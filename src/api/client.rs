use reqwest::Client;
use serde_json::{json, Value};
use crate::core::llm_config::LlmProvider;

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    /// Récupère et filtre la liste des modèles pour ne garder que le CHAT
    pub async fn fetch_models(&self, key: &str, provider: LlmProvider) -> Result<Vec<String>, String> {
        match provider {
            LlmProvider::Gemini => {
                let url = format!("https://generativelanguage.googleapis.com/v1beta/models?key={}", key);
                let resp = self.client.get(url).send().await.map_err(|e| e.to_string())?;
                let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                
                if let Some(models) = json["models"].as_array() {
                    let filtered = models.iter()
                        .filter_map(|m| {
                            let name = m["name"].as_str().unwrap_or("");
                            
                            // 1. Vérifier si le modèle supporte la génération de contenu (Chat)
                            let methods = m["supportedGenerationMethods"].as_array()
                                .map(|a| a.iter().any(|v| v == "generateContent")).unwrap_or(false);

                            // 2. Blacklist des mots-clés inutiles pour Sum Sum
                            let is_garbage = name.contains("banana")    // Image
                                || name.contains("lyria")              // Musique
                                || name.contains("robotics")           // Robotique
                                || name.contains("computer-use")       // Automatisation PC
                                || name.contains("embedding")          // Recherche vectorielle
                                || name.contains("aqa")                // Question Answering spécifique
                                || name.contains("vision")             // Si on veut que du texte pur
                                || name.contains("tts");               // Synthèse vocale

                            if methods && !is_garbage {
                                Some(name.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                    Ok(filtered)
                } else { 
                    Err("Clé invalide ou erreur de réponse Google".into()) 
                }
            },
            LlmProvider::OpenAI => {
                let resp = self.client.get("https://api.openai.com/v1/models")
                    .header("Authorization", format!("Bearer {}", key))
                    .send().await.map_err(|e| e.to_string())?;
                let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                
                if let Some(data) = json["data"].as_array() {
                    let filtered = data.iter()
                        .filter_map(|m| {
                            let id = m["id"].as_str()?;
                            // On ne garde que les familles GPT et O (raisonnement)
                            let is_chat = id.starts_with("gpt-") || id.starts_with("o1") || id.starts_with("o3") || id.starts_with("o4");
                            
                            // On exclut les variantes audio/realtime/instruct
                            let is_specialized = id.contains("audio") 
                                || id.contains("realtime") 
                                || id.contains("instruct")
                                || id.contains("search")
                                || id.contains("dall-e")
                                || id.contains("tts");

                            if is_chat && !is_specialized {
                                Some(id.to_string())
                            } else {
                                None
                            }
                        }).collect();
                    Ok(filtered)
                } else { 
                    Err("Erreur de réponse OpenAI".into()) 
                }
            },
            _ => Err("Provider non supporté pour le listage".into()),
        }
    }

    /// Envoie la conversation complète au modèle choisi
    pub async fn send_chat(&self, key: &str, provider: LlmProvider, model: &str, history: Vec<(String, String)>) -> Result<String, String> {
        let url = provider.get_api_url(model, key);
        
        let body = match provider {
            LlmProvider::Gemini => {
                // Gemini attend une structure "contents" avec "role" (user ou model)
                let contents: Vec<Value> = history.iter().map(|(role, content)| {
                    json!({
                        "role": if role.to_lowercase() == "user" { "user" } else { "model" }, 
                        "parts": [{"text": content}]
                    })
                }).collect();
                
                // On injecte le System Prompt comme instruction système (si supporté par le modèle)
                json!({
                    "contents": contents,
                    "systemInstruction": {
                        "parts": [{"text": crate::core::llm_config::SYSTEM_PROMPT}]
                    }
                })
            },
            LlmProvider::OpenAI => {
                let mut messages = vec![json!({"role": "system", "content": crate::core::llm_config::SYSTEM_PROMPT})];
                for (role, content) in history {
                    let api_role = if role.to_lowercase() == "user" { "user" } else { "assistant" };
                    messages.push(json!({"role": api_role, "content": content}));
                }
                json!({ "model": model, "messages": messages })
            },
            _ => return Err("Provider non implémenté pour le chat".into()),
        };

        let resp = self.client.post(url).json(&body).send().await.map_err(|e| e.to_string())?;
        
        // Gestion des erreurs HTTP (401, 403, 429...)
        if !resp.status().is_success() {
            let err_body: Value = resp.json().await.unwrap_or(json!({}));
            return Err(format!("API Error: {}", err_body["error"]["message"].as_str().unwrap_or("Unknown error")));
        }

        let json: Value = resp.json().await.map_err(|e| e.to_string())?;

        match provider {
            LlmProvider::Gemini => {
                json["candidates"][0]["content"]["parts"][0]["text"]
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| "Gemini a renvoyé une réponse vide (vérifiez vos filtres de sécurité)".to_string())
            },
            LlmProvider::OpenAI => {
                json["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| "OpenAI a renvoyé une réponse vide".to_string())
            },
            _ => Err("Erreur lors du parsing de la réponse".into()),
        }
    }
}