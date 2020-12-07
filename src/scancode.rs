#![allow(missing_docs)]

/// The SDL keyboard scancode representation.
///
/// This is used in places like the [`SDL_Keysym`] struct.
///
/// The values in this enumeration are based on the USB usage page standard:
/// https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf
///
/// See all the constants named `SDL_SCANCODE_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_Scancode(pub u32);

pub const SDL_SCANCODE_A: SDL_Scancode = SDL_Scancode(4);
pub const SDL_SCANCODE_B: SDL_Scancode = SDL_Scancode(5);
pub const SDL_SCANCODE_C: SDL_Scancode = SDL_Scancode(6);
pub const SDL_SCANCODE_D: SDL_Scancode = SDL_Scancode(7);
pub const SDL_SCANCODE_E: SDL_Scancode = SDL_Scancode(8);
pub const SDL_SCANCODE_F: SDL_Scancode = SDL_Scancode(9);
pub const SDL_SCANCODE_G: SDL_Scancode = SDL_Scancode(10);
pub const SDL_SCANCODE_H: SDL_Scancode = SDL_Scancode(11);
pub const SDL_SCANCODE_I: SDL_Scancode = SDL_Scancode(12);
pub const SDL_SCANCODE_J: SDL_Scancode = SDL_Scancode(13);
pub const SDL_SCANCODE_K: SDL_Scancode = SDL_Scancode(14);
pub const SDL_SCANCODE_L: SDL_Scancode = SDL_Scancode(15);
pub const SDL_SCANCODE_M: SDL_Scancode = SDL_Scancode(16);
pub const SDL_SCANCODE_N: SDL_Scancode = SDL_Scancode(17);
pub const SDL_SCANCODE_O: SDL_Scancode = SDL_Scancode(18);
pub const SDL_SCANCODE_P: SDL_Scancode = SDL_Scancode(19);
pub const SDL_SCANCODE_Q: SDL_Scancode = SDL_Scancode(20);
pub const SDL_SCANCODE_R: SDL_Scancode = SDL_Scancode(21);
pub const SDL_SCANCODE_S: SDL_Scancode = SDL_Scancode(22);
pub const SDL_SCANCODE_T: SDL_Scancode = SDL_Scancode(23);
pub const SDL_SCANCODE_U: SDL_Scancode = SDL_Scancode(24);
pub const SDL_SCANCODE_V: SDL_Scancode = SDL_Scancode(25);
pub const SDL_SCANCODE_W: SDL_Scancode = SDL_Scancode(26);
pub const SDL_SCANCODE_X: SDL_Scancode = SDL_Scancode(27);
pub const SDL_SCANCODE_Y: SDL_Scancode = SDL_Scancode(28);
pub const SDL_SCANCODE_Z: SDL_Scancode = SDL_Scancode(29);
pub const SDL_SCANCODE_1: SDL_Scancode = SDL_Scancode(30);
pub const SDL_SCANCODE_2: SDL_Scancode = SDL_Scancode(31);
pub const SDL_SCANCODE_3: SDL_Scancode = SDL_Scancode(32);
pub const SDL_SCANCODE_4: SDL_Scancode = SDL_Scancode(33);
pub const SDL_SCANCODE_5: SDL_Scancode = SDL_Scancode(34);
pub const SDL_SCANCODE_6: SDL_Scancode = SDL_Scancode(35);
pub const SDL_SCANCODE_7: SDL_Scancode = SDL_Scancode(36);
pub const SDL_SCANCODE_8: SDL_Scancode = SDL_Scancode(37);
pub const SDL_SCANCODE_9: SDL_Scancode = SDL_Scancode(38);
pub const SDL_SCANCODE_0: SDL_Scancode = SDL_Scancode(39);
pub const SDL_SCANCODE_RETURN: SDL_Scancode = SDL_Scancode(40);
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = SDL_Scancode(41);
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = SDL_Scancode(42);
pub const SDL_SCANCODE_TAB: SDL_Scancode = SDL_Scancode(43);
pub const SDL_SCANCODE_SPACE: SDL_Scancode = SDL_Scancode(44);
pub const SDL_SCANCODE_MINUS: SDL_Scancode = SDL_Scancode(45);
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = SDL_Scancode(46);
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = SDL_Scancode(47);
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = SDL_Scancode(48);
/// `\`
///
/// Located at the lower left of the return key on ISO keyboards and at the
/// right end of the QWERTY row on ANSI keyboards. Produces REVERSE SOLIDUS
/// (backslash) and VERTICAL LINE in a US layout, REVERSE SOLIDUS and VERTICAL
/// LINE in a UK Mac layout, NUMBER SIGN and TILDE in a UK Windows layout,
/// DOLLAR SIGN and POUND SIGN in a Swiss German layout, NUMBER SIGN and
/// APOSTROPHE in a German layout, GRAVE ACCENT and POUND SIGN in a French Mac
/// layout, and ASTERISK and MICRO SIGN in a French Windows layout.
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = SDL_Scancode(49);
/// Basically `\`, if you see it at all.
///
/// ISO USB keyboards actually use this code instead of 49 for the same key, but
/// all OSes I've seen treat the two codes identically. So, as an implementor,
/// unless your keyboard generates both of those codes and your OS treats them
/// differently, you should generate SDL_SCANCODE_BACKSLASH instead of this
/// code. As a user, you should not rely on this code because SDL will never
/// generate it with most (all?) keyboards.
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = SDL_Scancode(50);
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = SDL_Scancode(51);
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = SDL_Scancode(52);
/// <tt>\`</tt> character
///
/// Located in the top left corner (on both ANSI and ISO keyboards). Produces
/// GRAVE ACCENT and TILDE in a US Windows layout and in US and UK Mac layouts
/// on ANSI keyboards, GRAVE ACCENT and NOT SIGN in a UK Windows layout, SECTION
/// SIGN and PLUS-MINUS SIGN in US and UK Mac layouts on ISO keyboards, SECTION
/// SIGN and DEGREE SIGN in a Swiss German layout (Mac: only on ISO keyboards),
/// CIRCUMFLEX ACCENT and DEGREE SIGN in a German layout (Mac: only on ISO
/// keyboards), SUPERSCRIPT TWO and TILDE in a French Windows layout, COMMERCIAL
/// AT and NUMBER SIGN in a French Mac layout on ISO keyboards, and LESS-THAN
/// SIGN and GREATER-THAN SIGN in a Swiss German, German, or French Mac layout
/// on ANSI keyboards.
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = SDL_Scancode(53);
pub const SDL_SCANCODE_COMMA: SDL_Scancode = SDL_Scancode(54);
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = SDL_Scancode(55);
pub const SDL_SCANCODE_SLASH: SDL_Scancode = SDL_Scancode(56);
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = SDL_Scancode(57);
pub const SDL_SCANCODE_F1: SDL_Scancode = SDL_Scancode(58);
pub const SDL_SCANCODE_F2: SDL_Scancode = SDL_Scancode(59);
pub const SDL_SCANCODE_F3: SDL_Scancode = SDL_Scancode(60);
pub const SDL_SCANCODE_F4: SDL_Scancode = SDL_Scancode(61);
pub const SDL_SCANCODE_F5: SDL_Scancode = SDL_Scancode(62);
pub const SDL_SCANCODE_F6: SDL_Scancode = SDL_Scancode(63);
pub const SDL_SCANCODE_F7: SDL_Scancode = SDL_Scancode(64);
pub const SDL_SCANCODE_F8: SDL_Scancode = SDL_Scancode(65);
pub const SDL_SCANCODE_F9: SDL_Scancode = SDL_Scancode(66);
pub const SDL_SCANCODE_F10: SDL_Scancode = SDL_Scancode(67);
pub const SDL_SCANCODE_F11: SDL_Scancode = SDL_Scancode(68);
pub const SDL_SCANCODE_F12: SDL_Scancode = SDL_Scancode(69);
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = SDL_Scancode(70);
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = SDL_Scancode(71);
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = SDL_Scancode(72);
/// insert on PC, help on some Mac keyboards (but does send code 73, not 117)
pub const SDL_SCANCODE_INSERT: SDL_Scancode = SDL_Scancode(73);
pub const SDL_SCANCODE_HOME: SDL_Scancode = SDL_Scancode(74);
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = SDL_Scancode(75);
pub const SDL_SCANCODE_DELETE: SDL_Scancode = SDL_Scancode(76);
pub const SDL_SCANCODE_END: SDL_Scancode = SDL_Scancode(77);
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = SDL_Scancode(78);
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = SDL_Scancode(79);
pub const SDL_SCANCODE_LEFT: SDL_Scancode = SDL_Scancode(80);
pub const SDL_SCANCODE_DOWN: SDL_Scancode = SDL_Scancode(81);
pub const SDL_SCANCODE_UP: SDL_Scancode = SDL_Scancode(82);
/// num lock on PC, clear on Mac keyboards
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = SDL_Scancode(83);
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = SDL_Scancode(84);
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = SDL_Scancode(85);
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = SDL_Scancode(86);
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = SDL_Scancode(87);
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = SDL_Scancode(88);
pub const SDL_SCANCODE_KP_1: SDL_Scancode = SDL_Scancode(89);
pub const SDL_SCANCODE_KP_2: SDL_Scancode = SDL_Scancode(90);
pub const SDL_SCANCODE_KP_3: SDL_Scancode = SDL_Scancode(91);
pub const SDL_SCANCODE_KP_4: SDL_Scancode = SDL_Scancode(92);
pub const SDL_SCANCODE_KP_5: SDL_Scancode = SDL_Scancode(93);
pub const SDL_SCANCODE_KP_6: SDL_Scancode = SDL_Scancode(94);
pub const SDL_SCANCODE_KP_7: SDL_Scancode = SDL_Scancode(95);
pub const SDL_SCANCODE_KP_8: SDL_Scancode = SDL_Scancode(96);
pub const SDL_SCANCODE_KP_9: SDL_Scancode = SDL_Scancode(97);
pub const SDL_SCANCODE_KP_0: SDL_Scancode = SDL_Scancode(98);
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = SDL_Scancode(99);
/// varying
///
/// This is the additional key that ISO keyboards have over ANSI ones, located
/// between left shift and Y. Produces GRAVE ACCENT and TILDE in a US or UK Mac
/// layout, REVERSE SOLIDUS (backslash) and VERTICAL LINE in a US or UK Windows
/// layout, and LESS-THAN SIGN and GREATER-THAN SIGN in a Swiss German, German,
/// or French layout.
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = SDL_Scancode(100);
/// windows contextual menu, compose
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = SDL_Scancode(101);
/// The USB document says this is a status flag, not a physical key - but some
/// Mac keyboards do have a power key.
pub const SDL_SCANCODE_POWER: SDL_Scancode = SDL_Scancode(102);
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = SDL_Scancode(103);
pub const SDL_SCANCODE_F13: SDL_Scancode = SDL_Scancode(104);
pub const SDL_SCANCODE_F14: SDL_Scancode = SDL_Scancode(105);
pub const SDL_SCANCODE_F15: SDL_Scancode = SDL_Scancode(106);
pub const SDL_SCANCODE_F16: SDL_Scancode = SDL_Scancode(107);
pub const SDL_SCANCODE_F17: SDL_Scancode = SDL_Scancode(108);
pub const SDL_SCANCODE_F18: SDL_Scancode = SDL_Scancode(109);
pub const SDL_SCANCODE_F19: SDL_Scancode = SDL_Scancode(110);
pub const SDL_SCANCODE_F20: SDL_Scancode = SDL_Scancode(111);
pub const SDL_SCANCODE_F21: SDL_Scancode = SDL_Scancode(112);
pub const SDL_SCANCODE_F22: SDL_Scancode = SDL_Scancode(113);
pub const SDL_SCANCODE_F23: SDL_Scancode = SDL_Scancode(114);
pub const SDL_SCANCODE_F24: SDL_Scancode = SDL_Scancode(115);
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = SDL_Scancode(116);
pub const SDL_SCANCODE_HELP: SDL_Scancode = SDL_Scancode(117);
pub const SDL_SCANCODE_MENU: SDL_Scancode = SDL_Scancode(118);
pub const SDL_SCANCODE_SELECT: SDL_Scancode = SDL_Scancode(119);
pub const SDL_SCANCODE_STOP: SDL_Scancode = SDL_Scancode(120);
/// redo
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = SDL_Scancode(121);
pub const SDL_SCANCODE_UNDO: SDL_Scancode = SDL_Scancode(122);
pub const SDL_SCANCODE_CUT: SDL_Scancode = SDL_Scancode(123);
pub const SDL_SCANCODE_COPY: SDL_Scancode = SDL_Scancode(124);
pub const SDL_SCANCODE_PASTE: SDL_Scancode = SDL_Scancode(125);
pub const SDL_SCANCODE_FIND: SDL_Scancode = SDL_Scancode(126);
pub const SDL_SCANCODE_MUTE: SDL_Scancode = SDL_Scancode(127);
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = SDL_Scancode(128);
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = SDL_Scancode(129);
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = SDL_Scancode(133);
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = SDL_Scancode(134);
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = SDL_Scancode(135);
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = SDL_Scancode(136);
/// Yen
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = SDL_Scancode(137);
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = SDL_Scancode(138);
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = SDL_Scancode(139);
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = SDL_Scancode(140);
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = SDL_Scancode(141);
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = SDL_Scancode(142);
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = SDL_Scancode(143);
/// Hangul/English toggle
pub const SDL_SCANCODE_LANG1: SDL_Scancode = SDL_Scancode(144);
/// Hanja conversion
pub const SDL_SCANCODE_LANG2: SDL_Scancode = SDL_Scancode(145);
/// Katakana
pub const SDL_SCANCODE_LANG3: SDL_Scancode = SDL_Scancode(146);
/// Hiragana
pub const SDL_SCANCODE_LANG4: SDL_Scancode = SDL_Scancode(147);
/// Zenkaku/Hankaku
pub const SDL_SCANCODE_LANG5: SDL_Scancode = SDL_Scancode(148);
/// reserved
pub const SDL_SCANCODE_LANG6: SDL_Scancode = SDL_Scancode(149);
/// reserved
pub const SDL_SCANCODE_LANG7: SDL_Scancode = SDL_Scancode(150);
/// reserved
pub const SDL_SCANCODE_LANG8: SDL_Scancode = SDL_Scancode(151);
/// reserved
pub const SDL_SCANCODE_LANG9: SDL_Scancode = SDL_Scancode(152);
/// Erase-Eaze
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = SDL_Scancode(153);
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = SDL_Scancode(154);
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = SDL_Scancode(155);
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = SDL_Scancode(156);
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = SDL_Scancode(157);
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = SDL_Scancode(158);
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = SDL_Scancode(159);
pub const SDL_SCANCODE_OUT: SDL_Scancode = SDL_Scancode(160);
pub const SDL_SCANCODE_OPER: SDL_Scancode = SDL_Scancode(161);
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = SDL_Scancode(162);
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = SDL_Scancode(163);
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = SDL_Scancode(164);
pub const SDL_SCANCODE_KP_00: SDL_Scancode = SDL_Scancode(176);
pub const SDL_SCANCODE_KP_000: SDL_Scancode = SDL_Scancode(177);
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = SDL_Scancode(178);
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = SDL_Scancode(179);
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = SDL_Scancode(180);
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = SDL_Scancode(181);
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = SDL_Scancode(182);
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = SDL_Scancode(183);
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = SDL_Scancode(184);
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = SDL_Scancode(185);
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = SDL_Scancode(186);
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = SDL_Scancode(187);
pub const SDL_SCANCODE_KP_A: SDL_Scancode = SDL_Scancode(188);
pub const SDL_SCANCODE_KP_B: SDL_Scancode = SDL_Scancode(189);
pub const SDL_SCANCODE_KP_C: SDL_Scancode = SDL_Scancode(190);
pub const SDL_SCANCODE_KP_D: SDL_Scancode = SDL_Scancode(191);
pub const SDL_SCANCODE_KP_E: SDL_Scancode = SDL_Scancode(192);
pub const SDL_SCANCODE_KP_F: SDL_Scancode = SDL_Scancode(193);
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = SDL_Scancode(194);
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = SDL_Scancode(195);
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = SDL_Scancode(196);
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = SDL_Scancode(197);
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = SDL_Scancode(198);
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = SDL_Scancode(199);
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = SDL_Scancode(200);
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = SDL_Scancode(201);
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = SDL_Scancode(202);
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = SDL_Scancode(203);
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = SDL_Scancode(204);
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = SDL_Scancode(205);
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = SDL_Scancode(206);
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = SDL_Scancode(207);
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = SDL_Scancode(208);
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = SDL_Scancode(209);
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = SDL_Scancode(210);
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = SDL_Scancode(211);
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = SDL_Scancode(212);
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = SDL_Scancode(213);
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = SDL_Scancode(214);
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = SDL_Scancode(215);
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = SDL_Scancode(216);
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = SDL_Scancode(217);
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = SDL_Scancode(218);
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = SDL_Scancode(219);
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = SDL_Scancode(220);
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = SDL_Scancode(221);
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = SDL_Scancode(224);
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = SDL_Scancode(225);
/// alt, option
pub const SDL_SCANCODE_LALT: SDL_Scancode = SDL_Scancode(226);
/// windows, command (apple), meta
pub const SDL_SCANCODE_LGUI: SDL_Scancode = SDL_Scancode(227);
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = SDL_Scancode(228);
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = SDL_Scancode(229);
/// alt gr, option
pub const SDL_SCANCODE_RALT: SDL_Scancode = SDL_Scancode(230);
/// windows, command (apple), meta
pub const SDL_SCANCODE_RGUI: SDL_Scancode = SDL_Scancode(231);
/// I'm not sure if this is really not covered by any of the above, but since
/// there's a special KMOD_MODE for it I'm adding it here
pub const SDL_SCANCODE_MODE: SDL_Scancode = SDL_Scancode(257);
pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = SDL_Scancode(258);
pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = SDL_Scancode(259);
pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = SDL_Scancode(260);
pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = SDL_Scancode(261);
pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = SDL_Scancode(262);
pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = SDL_Scancode(263);
pub const SDL_SCANCODE_WWW: SDL_Scancode = SDL_Scancode(264);
pub const SDL_SCANCODE_MAIL: SDL_Scancode = SDL_Scancode(265);
pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = SDL_Scancode(266);
pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = SDL_Scancode(267);
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = SDL_Scancode(268);
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = SDL_Scancode(269);
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = SDL_Scancode(270);
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = SDL_Scancode(271);
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = SDL_Scancode(272);
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = SDL_Scancode(273);
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = SDL_Scancode(274);
pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = SDL_Scancode(275);
pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = SDL_Scancode(276);
/// display mirroring/dual display switch, video mode switch
pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = SDL_Scancode(277);
pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = SDL_Scancode(278);
pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = SDL_Scancode(279);
pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = SDL_Scancode(280);
pub const SDL_SCANCODE_EJECT: SDL_Scancode = SDL_Scancode(281);
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = SDL_Scancode(282);
pub const SDL_SCANCODE_APP1: SDL_Scancode = SDL_Scancode(283);
pub const SDL_SCANCODE_APP2: SDL_Scancode = SDL_Scancode(284);
pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = SDL_Scancode(285);
pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = SDL_Scancode(286);

/// Marks the number of scancodes, for array sizes and such.
pub const SDL_NUM_SCANCODES: usize = 512;
