#include <napi.h>

typedef struct TSLanguage TSLanguage;

extern "C" TSLanguage *tree_sitter_messageformat();

// "tree-sitter", "language" hashed with BLAKE2
const napi_type_tag LANGUAGE_TYPE_TAG = {
    0x8AF2E5212AD58ABF, 0xD5006CAD83ABBA16
};

Napi::Value GetLanguage(const Napi::CallbackInfo& info) {
    Napi::Env env = info.Env();
    auto language = Napi::External<TSLanguage>::New(env, tree_sitter_messageformat());
    language.TypeTag(&LANGUAGE_TYPE_TAG);
    return language;
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
    exports["language"] = Napi::Function::New(env, GetLanguage);
    return exports;
}

NODE_API_MODULE(tree_sitter_messageformat_binding, Init)
