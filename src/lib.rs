use zed_extension_api as zed;

struct MiseExtension;

impl zed::Extension for MiseExtension {
    fn new() -> Self {
        MiseExtension
    }
}

zed::register_extension!(MiseExtension);
