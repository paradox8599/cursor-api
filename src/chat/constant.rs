use super::model::Model;

macro_rules! def_pub_const {
    ($name:ident, $value:expr) => {
        pub const $name: &'static str = $value;
    };
}
def_pub_const!(ERR_UNSUPPORTED_GIF, "不支持动态 GIF");
def_pub_const!(
    ERR_UNSUPPORTED_IMAGE_FORMAT,
    "不支持的图片格式，仅支持 PNG、JPEG、WEBP 和非动态 GIF"
);
def_pub_const!(ERR_NODATA, "No data");

const MODEL_OBJECT: &str = "model";
const CREATED: &i64 = &1706659200;

def_pub_const!(ANTHROPIC, "anthropic");
def_pub_const!(CURSOR, "cursor");
def_pub_const!(GOOGLE, "google");
def_pub_const!(OPENAI, "openai");
def_pub_const!(DEEPSEEK, "deepseek");

def_pub_const!(CLAUDE_3_5_SONNET, "claude-3.5-sonnet");
def_pub_const!(GPT_4, "gpt-4");
def_pub_const!(GPT_4O, "gpt-4o");
def_pub_const!(CLAUDE_3_OPUS, "claude-3-opus");
def_pub_const!(CURSOR_FAST, "cursor-fast");
def_pub_const!(CURSOR_SMALL, "cursor-small");
def_pub_const!(GPT_3_5_TURBO, "gpt-3.5-turbo");
def_pub_const!(GPT_4_TURBO_2024_04_09, "gpt-4-turbo-2024-04-09");
def_pub_const!(GPT_4O_128K, "gpt-4o-128k");
def_pub_const!(GEMINI_1_5_FLASH_500K, "gemini-1.5-flash-500k");
def_pub_const!(CLAUDE_3_HAIKU_200K, "claude-3-haiku-200k");
def_pub_const!(CLAUDE_3_5_SONNET_200K, "claude-3-5-sonnet-200k");
def_pub_const!(CLAUDE_3_5_SONNET_20241022, "claude-3-5-sonnet-20241022");
def_pub_const!(GPT_4O_MINI, "gpt-4o-mini");
def_pub_const!(O1_MINI, "o1-mini");
def_pub_const!(O1_PREVIEW, "o1-preview");
def_pub_const!(O1, "o1");
def_pub_const!(CLAUDE_3_5_HAIKU, "claude-3.5-haiku");
def_pub_const!(GEMINI_EXP_1206, "gemini-exp-1206");
def_pub_const!(
    GEMINI_2_0_FLASH_THINKING_EXP,
    "gemini-2.0-flash-thinking-exp"
);
def_pub_const!(GEMINI_2_0_FLASH_EXP, "gemini-2.0-flash-exp");
def_pub_const!(DEEPSEEK_V3, "deepseek-v3");
def_pub_const!(DEEPSEEK_R1, "deepseek-r1");
def_pub_const!(CLAUDE_3_7_SONNET, "claude-3.7-sonnet");
def_pub_const!(CLAUDE_3_7_SONNET_THINKING, "claude-3.7-sonnet-thinking");

// #[derive(Clone, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
// pub enum ModelType {
//     Claude35Sonnet,
//     Gpt4,
//     Gpt4o,
//     Claude3Opus,
//     CursorFast,
//     CursorSmall,
//     Gpt35Turbo,
//     Gpt4Turbo202404,
//     Gpt4o128k,
//     Gemini15Flash500k,
//     Claude3Haiku200k,
//     Claude35Sonnet200k,
//     Claude35Sonnet20241022,
//     Gpt4oMini,
//     O1Mini,
//     O1Preview,
//     O1,
//     Claude35Haiku,
//     GeminiExp1206,
//     Gemini20FlashThinkingExp,
//     Gemini20FlashExp,
//     DeepseekV3,
//     DeepseekR1,
// }

macro_rules! create_model {
    ($($id:expr, $owner:expr),* $(,)?) => {
        pub const AVAILABLE_MODELS: [Model; count!($( ($id, $owner) )*)] = [
            $(
                Model {
                    id: $id,
                    created: CREATED,
                    object: MODEL_OBJECT,
                    owned_by: $owner,
                },
            )*
        ];
    };
}

macro_rules! count {
    () => (0);
    (($id:expr, $owner:expr) $( ($id2:expr, $owner2:expr) )*) => (1 + count!($( ($id2, $owner2) )*));
}

// impl ModelType {
//     pub fn as_str_name(&self) -> &'static str {
//         match self {
//             ModelType::Claude35Sonnet => CLAUDE_3_5_SONNET,
//             ModelType::Gpt4 => GPT_4,
//             ModelType::Gpt4o => GPT_4O,
//             ModelType::Claude3Opus => CLAUDE_3_OPUS,
//             ModelType::CursorFast => CURSOR_FAST,
//             ModelType::CursorSmall => CURSOR_SMALL,
//             ModelType::Gpt35Turbo => GPT_3_5_TURBO,
//             ModelType::Gpt4Turbo202404 => GPT_4_TURBO_2024_04_09,
//             ModelType::Gpt4o128k => GPT_4O_128K,
//             ModelType::Gemini15Flash500k => GEMINI_1_5_FLASH_500K,
//             ModelType::Claude3Haiku200k => CLAUDE_3_HAIKU_200K,
//             ModelType::Claude35Sonnet200k => CLAUDE_3_5_SONNET_200K,
//             ModelType::Claude35Sonnet20241022 => CLAUDE_3_5_SONNET_20241022,
//             ModelType::Gpt4oMini => GPT_4O_MINI,
//             ModelType::O1Mini => O1_MINI,
//             ModelType::O1Preview => O1_PREVIEW,
//             ModelType::O1 => O1,
//             ModelType::Claude35Haiku => CLAUDE_3_5_HAIKU,
//             ModelType::GeminiExp1206 => GEMINI_EXP_1206,
//             ModelType::Gemini20FlashThinkingExp => GEMINI_2_0_FLASH_THINKING_EXP,
//             ModelType::Gemini20FlashExp => GEMINI_2_0_FLASH_EXP,
//             ModelType::DeepseekV3 => DEEPSEEK_V3,
//             ModelType::DeepseekR1 => DEEPSEEK_R1,
//         }
//     }

//     pub fn from_str_name(id :&str) -> Option<ModelType> {
//         match id {
//             CLAUDE_3_5_SONNET => Some(ModelType::Claude35Sonnet),
//             GPT_4 => Some(ModelType::Gpt4),
//             GPT_4O => Some(ModelType::Gpt4o),
//             CLAUDE_3_OPUS => Some(ModelType::Claude3Opus),
//             CURSOR_FAST => Some(ModelType::CursorFast),
//             CURSOR_SMALL => Some(ModelType::CursorSmall),
//             GPT_3_5_TURBO => Some(ModelType::Gpt35Turbo),
//             GPT_4_TURBO_2024_04_09 => Some(ModelType::Gpt4Turbo202404),
//             GPT_4O_128K => Some(ModelType::Gpt4o128k),
//             GEMINI_1_5_FLASH_500K => Some(ModelType::Gemini15Flash500k),
//             CLAUDE_3_HAIKU_200K => Some(ModelType::Claude3Haiku200k),
//             CLAUDE_3_5_SONNET_200K => Some(ModelType::Claude35Sonnet200k),
//             CLAUDE_3_5_SONNET_20241022 => Some(ModelType::Claude35Sonnet20241022),
//             GPT_4O_MINI => Some(ModelType::Gpt4oMini),
//             O1_MINI => Some(ModelType::O1Mini),
//             O1_PREVIEW => Some(ModelType::O1Preview),
//             O1 => Some(ModelType::O1),
//             CLAUDE_3_5_HAIKU => Some(ModelType::Claude35Haiku),
//             GEMINI_EXP_1206 => Some(ModelType::GeminiExp1206),
//             GEMINI_2_0_FLASH_THINKING_EXP => Some(ModelType::Gemini20FlashThinkingExp),
//             GEMINI_2_0_FLASH_EXP => Some(ModelType::Gemini20FlashExp),
//             DEEPSEEK_V3 => Some(ModelType::DeepseekV3),
//             DEEPSEEK_R1 => Some(ModelType::DeepseekR1),
//             _ => None,
//         }
//     }
// }

create_model!(
    CLAUDE_3_5_SONNET, ANTHROPIC,
    GPT_4, OPENAI,
    GPT_4O, OPENAI,
    CLAUDE_3_OPUS, ANTHROPIC,
    CURSOR_FAST, CURSOR,
    CURSOR_SMALL, CURSOR,
    GPT_3_5_TURBO, OPENAI,
    GPT_4_TURBO_2024_04_09, OPENAI,
    GPT_4O_128K, OPENAI,
    GEMINI_1_5_FLASH_500K, GOOGLE,
    CLAUDE_3_HAIKU_200K, ANTHROPIC,
    CLAUDE_3_5_SONNET_200K, ANTHROPIC,
    CLAUDE_3_5_SONNET_20241022, ANTHROPIC,
    GPT_4O_MINI, OPENAI,
    O1_MINI, OPENAI,
    O1_PREVIEW, OPENAI,
    O1, OPENAI,
    CLAUDE_3_5_HAIKU, ANTHROPIC,
    GEMINI_EXP_1206, GOOGLE,
    GEMINI_2_0_FLASH_THINKING_EXP, GOOGLE,
    GEMINI_2_0_FLASH_EXP, GOOGLE,
    DEEPSEEK_V3, DEEPSEEK,
    DEEPSEEK_R1, DEEPSEEK,
    CLAUDE_3_7_SONNET, ANTHROPIC,
    CLAUDE_3_7_SONNET_THINKING, ANTHROPIC,
);

pub const USAGE_CHECK_MODELS: [&str; 13] = [
    CLAUDE_3_5_SONNET_20241022,
    CLAUDE_3_5_SONNET,
    GEMINI_EXP_1206,
    GPT_4,
    GPT_4_TURBO_2024_04_09,
    GPT_4O,
    CLAUDE_3_5_HAIKU,
    GPT_4O_128K,
    GEMINI_1_5_FLASH_500K,
    CLAUDE_3_HAIKU_200K,
    CLAUDE_3_5_SONNET_200K,
    CLAUDE_3_7_SONNET,
    CLAUDE_3_7_SONNET_THINKING,
];

pub const LONG_CONTEXT_MODELS: [&str; 6] = [
    GPT_4O_128K,
    GEMINI_1_5_FLASH_500K,
    CLAUDE_3_HAIKU_200K,
    CLAUDE_3_5_SONNET_200K,
    CLAUDE_3_7_SONNET,
    CLAUDE_3_7_SONNET_THINKING,
];

// include!("constant/models.rs");
