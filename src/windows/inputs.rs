use crate::public::{
    KeybdKey::{self, *},
    MouseButton::{self, *},
};

impl From<KeybdKey> for u64 {
    fn from(key: KeybdKey) -> u64 {
        match key {
            BackspaceKey => 0x08,
            TabKey => 0x09,
            EnterKey => 0x0D,
            EscapeKey => 0x1B,
            SpaceKey => 0x20,
            HomeKey => 0x24,
            LeftKey => 0x25,
            UpKey => 0x26,
            RightKey => 0x27,
            DownKey => 0x28,
            InsertKey => 0x2D,
            DeleteKey => 0x2E,
            Numrow0Key => 0x30,
            Numrow1Key => 0x31,
            Numrow2Key => 0x32,
            Numrow3Key => 0x33,
            Numrow4Key => 0x34,
            Numrow5Key => 0x35,
            Numrow6Key => 0x36,
            Numrow7Key => 0x37,
            Numrow8Key => 0x38,
            Numrow9Key => 0x39,
            AKey => 0x41,
            BKey => 0x42,
            CKey => 0x43,
            DKey => 0x44,
            EKey => 0x45,
            FKey => 0x46,
            GKey => 0x47,
            HKey => 0x48,
            IKey => 0x49,
            JKey => 0x4A,
            KKey => 0x4B,
            LKey => 0x4C,
            MKey => 0x4D,
            NKey => 0x4E,
            OKey => 0x4F,
            PKey => 0x50,
            QKey => 0x51,
            RKey => 0x52,
            SKey => 0x53,
            TKey => 0x54,
            UKey => 0x55,
            VKey => 0x56,
            WKey => 0x57,
            XKey => 0x58,
            YKey => 0x59,
            ZKey => 0x5A,
            //TODO: reset these on next reboot
            Numpad0Key => 0x60,
            Numpad1Key => 0x61,
            Numpad2Key => 0x62,
            Numpad3Key => 0x63,
            Numpad4Key => 0x64,
            Numpad5Key => 0x65,
            Numpad6Key => 0x66,
            Numpad7Key => 0x67,
            Numpad8Key => 0x68,
            Numpad9Key => 0x69,
            // Numpad0Key => 0x60,
            // Numpad1Key => 0xC1,
            // Numpad2Key => 0xE9,
            // Numpad3Key => 0xFF,
            // Numpad4Key => 0xEE,
            // Numpad5Key => 0xF1,
            // Numpad6Key => 0xEA,
            // Numpad7Key => 0xF9,
            // Numpad8Key => 0xF5,
            // Numpad9Key => 0xF3,
            NumpadDivKey => 0x6F,
            NumpadPlusKey => 0x6B,
            NumpadDelKey => 0x6E,
            NumpadEnterKey => 0x0D, //0xB0?
            F1Key => 0x70,
            F2Key => 0x71,
            F3Key => 0x72,
            F4Key => 0x73,
            F5Key => 0x74,
            F6Key => 0x75,
            F7Key => 0x76,
            F8Key => 0x77,
            F9Key => 0x78,
            F10Key => 0x79,
            F11Key => 0x7A,
            F12Key => 0x7B,
            F13Key => 0x7C,
            F14Key => 0x7D,
            F15Key => 0x7E,
            F16Key => 0x7F,
            F17Key => 0x80,
            F18Key => 0x81,
            F19Key => 0x82,
            F20Key => 0x83,
            F21Key => 0x84,
            F22Key => 0x85,
            F23Key => 0x86,
            F24Key => 0x87,
            NumLockKey => 0x90,
            ScrollLockKey => 0x91,
            CapsLockKey => 0x14,
            LShiftKey => 0xA0,
            RShiftKey => 0xA1,
            LControlKey => 0xA2,
            RControlKey => 0xA3,
            OtherKey(code) => code,
            MouseKeyUpperLeft => 900,
            MouseKeyUp => 901,
            MouseKeyUpperRight => 902,
            MouseKeyHistoryForward => 903,
            MouseKeyLeft => 904,
            MouseKeyMiddle => 905,
            MouseKeyRight => 906,
            MouseKeyHistoryBack => 907,
            MouseKeyLowerLeft => 908,
            MouseKeyDown => 909,
            MouseKeyLowerRight => 910,
            MouseKeyFast => 911,
            MouseKeySlow => 0xe01c,
            MouseKeyClickToggle => 912,
            MouseKeyActivate => 914,
            MouseKeySlash => 915,
            MouseKeyNumlock => 916,
        }
    }
}

impl From<u64> for KeybdKey {
    fn from(code: u64) -> KeybdKey {
        match code {
            0x08 => BackspaceKey,
            0x09 => TabKey,
            0x0D => EnterKey,
            0x1B => EscapeKey,
            0x20 => SpaceKey,
            0x24 => HomeKey,
            0x25 => LeftKey,
            0x26 => UpKey,
            0x27 => RightKey,
            0x28 => DownKey,
            0x2D => InsertKey,
            0x2E => DeleteKey,
            0x30 => Numrow0Key,
            0x31 => Numrow1Key,
            0x32 => Numrow2Key,
            0x33 => Numrow3Key,
            0x34 => Numrow4Key,
            0x35 => Numrow5Key,
            0x36 => Numrow6Key,
            0x37 => Numrow7Key,
            0x38 => Numrow8Key,
            0x39 => Numrow9Key,
            0x41 => AKey,
            0x42 => BKey,
            0x43 => CKey,
            0x44 => DKey,
            0x45 => EKey,
            0x46 => FKey,
            0x47 => GKey,
            0x48 => HKey,
            0x49 => IKey,
            0x4A => JKey,
            0x4B => KKey,
            0x4C => LKey,
            0x4D => MKey,
            0x4E => NKey,
            0x4F => OKey,
            0x50 => PKey,
            0x51 => QKey,
            0x52 => RKey,
            0x53 => SKey,
            0x54 => TKey,
            0x55 => UKey,
            0x56 => VKey,
            0x57 => WKey,
            0x58 => XKey,
            0x59 => YKey,
            0x5A => ZKey,
            0x60 => Numpad0Key,
            0x61 => Numpad1Key,
            0x62 => Numpad2Key,
            0x63 => Numpad3Key,
            0x64 => Numpad4Key,
            0x65 => Numpad5Key,
            0x66 => Numpad6Key,
            0x67 => Numpad7Key,
            0x68 => Numpad8Key,
            0x69 => Numpad9Key,
            // 0xC1 => Numpad1Key,
            // 0xE9 => Numpad2Key,
            // 0xFF => Numpad3Key,
            // 0xEE => Numpad4Key,
            // 0xF1 => Numpad5Key,
            // 0xEA => Numpad6Key,
            // 0xF9 => Numpad7Key,
            // 0xF5 => Numpad8Key,
            // 0xF3 => Numpad9Key,
            0x6B => NumpadPlusKey,
            0x6F => NumpadDivKey,
            0x6E => NumpadDelKey,
            0x0D => NumpadEnterKey,
            0x70 => F1Key,
            0x71 => F2Key,
            0x72 => F3Key,
            0x73 => F4Key,
            0x74 => F5Key,
            0x75 => F6Key,
            0x76 => F7Key,
            0x77 => F8Key,
            0x78 => F9Key,
            0x79 => F10Key,
            0x7A => F11Key,
            0x7B => F12Key,
            0x7C => F13Key,
            0x7D => F14Key,
            0x7E => F15Key,
            0x7F => F16Key,
            0x80 => F17Key,
            0x81 => F18Key,
            0x82 => F19Key,
            0x83 => F20Key,
            0x84 => F21Key,
            0x85 => F22Key,
            0x86 => F23Key,
            0x87 => F24Key,
            0x90 => NumLockKey,
            0x91 => ScrollLockKey,
            0x14 => CapsLockKey,
            0xA0 => LShiftKey,
            0xA1 => RShiftKey,
            0xA2 => LControlKey,
            0xA3 => RControlKey,
            900 => MouseKeyUpperLeft,
            901 => MouseKeyUp,
            902 => MouseKeyUpperRight,
            903 => MouseKeyHistoryForward,
            904 => MouseKeyLeft,
            905 => MouseKeyMiddle,
            906 => MouseKeyRight,
            907 => MouseKeyHistoryBack,
            908 => MouseKeyLowerLeft,
            909 => MouseKeyDown,
            910 => MouseKeyLowerRight,
            911 => MouseKeyFast,
            0xe01c => MouseKeySlow,
            912 => MouseKeyClickToggle,
            914 => MouseKeyActivate,
            915 => MouseKeySlash,
            916 => MouseKeyNumlock,
            _ => OtherKey(code),
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            LeftButton => 0x01,
            RightButton => 0x02,
            MiddleButton => 0x04,
            X1Button => 0x05,
            X2Button => 0x06,
            OtherButton(code) => code,
        }
    }
}
