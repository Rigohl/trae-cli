//! Internationalization module for Jarvix CLI
//! Provides multilanguage support for all CLI commands

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedText {
    pub en: String,
    pub es: String,
    pub fr: String,
    pub de: String,
    pub ja: String,
    pub pt: String,
}

impl LocalizedText {
    pub fn get(&self, lang: &str) -> &str {
        match lang {
            "es" => &self.es,
            "fr" => &self.fr,
            "de" => &self.de,
            "ja" => &self.ja,
            "pt" => &self.pt,
            _ => &self.en, // Default to English
        }
    }
    
    pub fn new(en: &str, es: &str, fr: &str, de: &str, ja: &str, pt: &str) -> Self {
        Self {
            en: en.to_string(),
            es: es.to_string(),
            fr: fr.to_string(),
            de: de.to_string(),
            ja: ja.to_string(),
            pt: pt.to_string(),
        }
    }
}

pub struct I18nProvider {
    texts: HashMap<String, LocalizedText>,
    current_language: String,
}

impl I18nProvider {
    pub fn new(default_lang: &str) -> Self {
        let mut provider = Self {
            texts: HashMap::new(),
            current_language: default_lang.to_string(),
        };
        
        // Initialize with common CLI texts
        provider.load_default_texts();
        provider
    }
    
    fn load_default_texts(&mut self) {
        // Common CLI messages
        self.texts.insert("welcome_msg".to_string(), LocalizedText::new(
            "Welcome to Jarvix CLI", 
            "Bienvenido a Jarvix CLI", 
            "Bienvenue à Jarvix CLI", 
            "Willkommen bei Jarvix CLI", 
            "Jarvix CLIへようこそ", 
            "Bem-vindo ao Jarvix CLI"
        ));
        
        self.texts.insert("error_occurred".to_string(), LocalizedText::new(
            "An error occurred", 
            "Ocurrió un error", 
            "Une erreur s'est produite", 
            "Ein Fehler ist aufgetreten", 
            "エラーが発生しました", 
            "Ocorreu um erro"
        ));
        
        self.texts.insert("command_executed".to_string(), LocalizedText::new(
            "Command executed successfully", 
            "Comando ejecutado exitosamente", 
            "Commande exécutée avec succès", 
            "Befehl erfolgreich ausgeführt", 
            "コマンドは正常に実行されました", 
            "Comando executado com sucesso"
        ));
        
        // Command descriptions
        self.texts.insert("scan_cmd_desc".to_string(), LocalizedText::new(
            "Multilanguage project analysis and scanning", 
            "Análisis y escaneo multilenguaje del proyecto", 
            "Analyse et analyse multi-langue du projet", 
            "Projektanalyse und -scanning in mehreren Sprachen", 
            "プロジェクトの多言語分析とスキャン", 
            "Análise e varredura multilíngue do projeto"
        ));
        
        self.texts.insert("analyze_cmd_desc".to_string(), LocalizedText::new(
            "Deep code analysis with mathematical models", 
            "Análisis profundo del código con modelos matemáticos", 
            "Analyse approfondie du code avec modèles mathématiques", 
            "Tiefgreifende Code-Analyse mit mathematischen Modellen", 
            "数学モデルによるコードの深層分析", 
            "Análise profunda de código com modelos matemáticos"
        ));
        
        self.texts.insert("math_cmd_desc".to_string(), LocalizedText::new(
            "Mathematical analysis with chaos theory", 
            "Análisis matemático con teoría del caos", 
            "Analyse mathématique avec théorie du chaos", 
            "Mathematische Analyse mit Chaostheorie", 
            "カオス理論による数学的分析", 
            "Análise matemática com teoria do caos"
        ));
    }
    
    pub fn set_language(&mut self, lang: &str) {
        self.current_language = lang.to_string();
    }
    
    pub fn t(&self, key: &str) -> &str {
        if let Some(text) = self.texts.get(key) {
            text.get(&self.current_language)
        } else {
            // Return English version if key not found
            if let Some(text) = self.texts.get(key) {
                text.get("en")
            } else {
                key
            }
        }
    }
    
    pub fn add_text(&mut self, key: &str, text: LocalizedText) {
        self.texts.insert(key.to_string(), text);
    }
}

// Global instance
static mut I18N_PROVIDER: Option<I18nProvider> = None;
static mut INITIALIZED: bool = false;

pub fn init_i18n(default_lang: &str) {
    unsafe {
        if !INITIALIZED {
            I18N_PROVIDER = Some(I18nProvider::new(default_lang));
            INITIALIZED = true;
        }
    }
}

pub fn t(key: &str) -> &'static str {
    unsafe {
        if let Some(provider) = &I18N_PROVIDER {
            provider.t(key)
        } else {
            key
        }
    }
}

pub fn set_language(lang: &str) {
    unsafe {
        if let Some(provider) = &mut I18N_PROVIDER {
            provider.set_language(lang);
        }
    }
}