use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Key {
    A,
    Ab,
    AddressBook,
    Again,
    AlsToggle,
    AltErase,
    Angle,
    Apostrophe,
    Appselect,
    Archive,
    AspectRatio,
    Assistant,
    AttendantOff,
    AttendantOn,
    AttendantToggle,
    Audio,
    AudioDesc,
    Aux,
    B,
    Back,
    Backslash,
    Backspace,
    BassBoost,
    Battery,
    Blue,
    Bluetooth,
    Bookmarks,
    Break,
    BrightnessAuto,
    BrightnessCycle,
    BrightnessMax,
    BrightnessMin,
    BrightnessToggle,
    BrightnessZero,
    BrightnessDown,
    BrightnessUp,
    BrlDot1,
    BrlDot10,
    BrlDot2,
    BrlDot3,
    BrlDot4,
    BrlDot5,
    BrlDot6,
    BrlDot7,
    BrlDot8,
    BrlDot9,
    ButtonConfig,
    C,
    Calc,
    Calendar,
    Camera,
    CameraDown,
    CameraFocus,
    CameraLeft,
    CameraRight,
    CameraUp,
    CameraZoomIn,
    CameraZoomOut,
    Cancel,
    CapsLock,
    Cd,
    Channel,
    ChannelDown,
    ChannelUp,
    Chat,
    Clear,
    Close,
    CloseCd,
    Coffee,
    Comma,
    Compose,
    Computer,
    Config,
    Connect,
    ContextMenu,
    Controlpanel,
    Copy,
    Cut,
    CycleWindows,
    D,
    Dashboard,
    Data,
    Database,
    DelEol,
    DelEos,
    DelLine,
    Delete,
    DeleteFile,
    Digits,
    Direction,
    Directory,
    DisplayOff,
    DisplayToggle,
    Documents,
    Dollar,
    Dot,
    Down,
    Dvd,
    E,
    Edit,
    Editor,
    EjectCd,
    EjectCloseCd,
    Email,
    End,
    Enter,
    Epg,
    Equal,
    Esc,
    Euro,
    Exit,
    F,
    F1,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F2,
    F20,
    F21,
    F22,
    F23,
    F24,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    FastForward,
    FastReverse,
    Favorites,
    File,
    Finance,
    Find,
    First,
    Fn,
    Fn1,
    Fn2,
    FnB,
    FnD,
    FnE,
    FnEsc,
    FnF,
    FnF1,
    FnF10,
    FnF11,
    FnF12,
    FnF2,
    FnF3,
    FnF4,
    FnF5,
    FnF6,
    FnF7,
    FnF8,
    FnF9,
    FnS,
    Forward,
    ForwardMail,
    Frameback,
    FrameForward,
    Front,
    FullScreen,
    G,
    Games,
    Goto,
    GraphicsEditor,
    Grave,
    Green,
    H,
    Hangeul,
    Hanja,
    Help,
    Henkan,
    Hiragana,
    Home,
    Homepage,
    Hp,
    I,
    Images,
    Info,
    InsLine,
    Insert,
    Iso,
    J,
    Journal,
    K,
    Katakana,
    KatakanaHiragana,
    KbdLayoutNext,
    KbdLcdMenu1,
    KbdLcdMenu2,
    KbdLcdMenu3,
    KbdLcdMenu4,
    KbdLcdMenu5,
    KbdIllumDown,
    KbdIllumToggle,
    KbdIllumUp,
    KbdInputAssistAccept,
    KbdInputAssistCancel,
    KbdInputAssistNext,
    KbdInputAssistNextgroup,
    KbdInputAssistPrev,
    KbdInputAssistPrevgroup,
    Keyboard,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpAsterisk,
    KpComma,
    KpDott,
    KpEnter,
    KpEqual,
    KpJpComma,
    KpLeftParen,
    KpMinus,
    KpPlus,
    KpPlusMinus,
    KpRightParen,
    KpSlash,
    L,
    Language,
    Last,
    Left,
    LeftDown,
    LeftUp,
    LeftAlt,
    LeftBrace,
    LeftCtrl,
    LeftMeta,
    LeftShift,
    LightsToggle,
    LineFeed,
    List,
    LogOff,
    M,
    Macro,
    Macro1,
    Macro10,
    Macro11,
    Macro12,
    Macro13,
    Macro14,
    Macro15,
    Macro16,
    Macro17,
    Macro18,
    Macro19,
    Macro2,
    Macro20,
    Macro21,
    Macro22,
    Macro23,
    Macro24,
    Macro25,
    Macro26,
    Macro27,
    Macro28,
    Macro29,
    Macro3,
    Macro30,
    Macro4,
    Macro5,
    Macro6,
    Macro7,
    Macro8,
    Macro9,
    MacroPreset1,
    MacroPreset2,
    MacroPreset3,
    MacroPresetCycle,
    MacroRecordStart,
    MacroRecordStop,
    Mail,
    Media,
    MediaRepeat,
    MediaTopMenu,
    Memo,
    Menu,
    Messenger,
    Mhp,
    MicMute,
    Minus,
    Mode,
    Move,
    Mp3,
    MsDos,
    Muhenkan,
    Mute,
    N,
    N0,
    N1,
    N102nd,
    N10ChannelsDown,
    N10ChannelsUp,
    N2,
    N3,
    N3dMode,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    New,
    News,
    Next,
    NextFavorite,
    Nextsong,
    Numeric0,
    Numeric1,
    Numeric11,
    Numeric12,
    Numeric2,
    Numeric3,
    Numeric4,
    Numeric5,
    Numeric6,
    Numeric7,
    Numeric8,
    Numeric9,
    NumericA,
    NumericB,
    NumericC,
    NumericD,
    NumericPound,
    NumericStar,
    NumLock,
    O,
    Ok,
    OnscreenKeyboard,
    Open,
    Option,
    P,
    Pagedown,
    Pageup,
    Paste,
    Pause,
    PauseRecord,
    PauseCd,
    Pc,
    Phone,
    Play,
    PlayCd,
    Player,
    PlayPause,
    Power,
    Power2,
    Presentation,
    Previous,
    PreviousSong,
    Print,
    PrivacyScreenToggle,
    Prog1,
    Prog2,
    Prog3,
    Prog4,
    Program,
    Props,
    Pvr,
    Q,
    Question,
    R,
    Radio,
    Record,
    Red,
    Redo,
    Refresh,
    Reply,
    Reserved,
    Restart,
    Rewind,
    RfKill,
    Right,
    RightDown,
    RightUp,
    RightAlt,
    RightBrace,
    RightCtrl,
    RightMeta,
    RightShift,
    Ro,
    RootMenu,
    RotateDisplay,
    RotateLockToggle,
    S,
    Sat,
    Sat2,
    Save,
    Scale,
    Screen,
    Screenlock,
    Screensaver,
    ScrollDown,
    ScrollLock,
    ScrollUp,
    Search,
    Select,
    SelectiveScreenshot,
    Semicolon,
    Send,
    SendFile,
    Setup,
    Shop,
    Shuffle,
    Slash,
    Sleep,
    Slow,
    SlowReverse,
    Sound,
    Space,
    Spellcheck,
    Sport,
    Spreadsheet,
    Stop,
    StopRecord,
    StopCd,
    Subtitle,
    Suspend,
    SwitchVideoMode,
    SysRq,
    T,
    Tab,
    Tape,
    TaskManager,
    Teen,
    Text,
    Time,
    Title,
    TouchpadOff,
    TouchpadOn,
    TouchpadToggle,
    Tuner,
    Tv,
    Tv2,
    Twen,
    U,
    Undo,
    Unknown,
    Unmute,
    Up,
    Uwb,
    V,
    Vcr,
    Vcr2,
    Vendor,
    Video,
    VideoNext,
    VideoPrev,
    VideoPhone,
    Vod,
    VoiceCommand,
    VoiceMail,
    VolumeDown,
    VolumeUp,
    W,
    WakeUp,
    Wimax,
    Wlan,
    WordProcessor,
    WpsButton,
    Wwan,
    Www,
    X,
    Xfer,
    Y,
    Yellow,
    Yen,
    Z,
    ZenkakuHankaku,
    Zoom,
    ZoomIn,
    ZoomOut,
    ZoomReset,
}
