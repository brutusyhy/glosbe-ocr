// src/lib/languages.ts
export type Lang = {
  code: string;          // ISO 639-1 code
  name: string;          // English name (display primary)
  nativeNames: string[]; // native spellings / common synonyms
};

export type LangOption = {
  value: string;
  label: string;
}

export const LANGUAGES: Lang[] = [
  { code: "af", name: "Afrikaans", nativeNames: ["Afrikaans"] },
  { code: "sq", name: "Albanian", nativeNames: ["shqip"] },
  { code: "ar", name: "Arabic", nativeNames: ["العربية","Arabic"] },
  { code: "hy", name: "Armenian", nativeNames: ["Armjanski Yazyk","Ena","Ermeni Dili","Ermenice","Somkhuri","հայերեն"] },
  { code: "az", name: "Azerbaijani", nativeNames: ["azərbaycan dili"] },
  { code: "bn", name: "Bangla", nativeNames: ["Bangala","Bangla-Bhasa","বাংলা"] },
  { code: "eu", name: "Basque", nativeNames: ["Euskara","Euskera","Vascuense","euskara"] },
  { code: "be", name: "Belarusian", nativeNames: ["Belorussian","Bielorussian","Byelorussian","White Russian","White Ruthenian","беларуская"] },
  { code: "bs", name: "Bosnian", nativeNames: ["bosanski"] },
  { code: "bg", name: "Bulgarian", nativeNames: ["Balgarski","български"] },
  { code: "my", name: "Burmese", nativeNames: ["Bama","Bamachaka","Myanmar","Myen","ဗမာ"] },
  { code: "ca", name: "Catalan", nativeNames: ["Català","Catalan-Valencian-Balear","Catalonian","català"] },
  { code: "zh", name: "Chinese", nativeNames: ["中文"] },
  { code: "hr", name: "Croatian", nativeNames: ["Hrvatski","hrvatski"] },
  { code: "cs", name: "Czech", nativeNames: ["Bohemian","Cestina","čeština"] },
  { code: "da", name: "Danish", nativeNames: ["Dansk","Rigsdansk","dansk"] },
  { code: "nl", name: "Dutch", nativeNames: ["Hollands","Nederlands"] },
  { code: "en", name: "English", nativeNames: ["English"] },
  { code: "eo", name: "Esperanto", nativeNames: ["Eo","La Lingvo Internacia"] },
  { code: "et", name: "Estonian", nativeNames: ["eesti"] },
  { code: "fil", name: "Filipino", nativeNames: ["Filipino"] },
  { code: "fi", name: "Finnish", nativeNames: ["Suomi","suomi"] },
  { code: "fr", name: "French", nativeNames: ["Français","français"] },
  { code: "gl", name: "Galician", nativeNames: ["Galego","Gallego","galego"] },
  { code: "ka", name: "Georgian", nativeNames: ["Common Kartvelian","Gruzinski","Kartuli","ქართული"] },
  { code: "de", name: "German", nativeNames: ["Tedesco","Deutsch"] },
  { code: "el", name: "Greek", nativeNames: ["Ellinika","Graecae","Grec","Greco","Neo-Hellenic","Romaic","Ελληνικά"] },
  { code: "gu", name: "Gujarati", nativeNames: ["Gujerathi","Gujerati","Gujrathi","ગુજરાતી"] },
  { code: "ht", name: "Haitian", nativeNames: ["Haitian Creole","Kreyòl ayisyen"] },
  { code: "he", name: "Hebrew", nativeNames: ["Israeli","Ivrit","עברית"] },
  { code: "hi", name: "Hindi", nativeNames: ["Khadi Boli","Khari Boli","हिन्दी"] },
  { code: "hu", name: "Hungarian", nativeNames: ["Magyar","magyar"] },
  { code: "io", name: "Ido", nativeNames: ["Ido"] },
  { code: "id", name: "Indonesian", nativeNames: ["Bahasa Indonesia","Indonesia"] },
  { code: "ia", name: "Interlingua", nativeNames: ["Interlingua"] },
  { code: "ga", name: "Irish", nativeNames: ["Erse","Gaelic Irish","Gaeilge"] },
  { code: "it", name: "Italian", nativeNames: ["Italiano","italiano"] },
  { code: "ja", name: "Japanese", nativeNames: ["日本語"] },
  { code: "kk", name: "Kazakh", nativeNames: ["Kaisak","Kazak","Kosach","Qazaq","Қазақ тілі"] },
  { code: "km", name: "Khmer", nativeNames: ["Cambodian","ខ្មែរ"] },
  { code: "ko", name: "Korean", nativeNames: ["Hanguk Mal","Hanguk Uh","한국어"] },
  { code: "ku", name: "Kurdish", nativeNames: ["Kurmanji","Zimanê kurdî"] },
  { code: "lo", name: "Lao", nativeNames: ["Eastern Thai","Lào","Laotian","ລາວ"] },
  { code: "la", name: "Latin", nativeNames: ["Latina"] },
  { code: "lv", name: "Latvian", nativeNames: ["latviešu"] },
  { code: "lt", name: "Lithuanian", nativeNames: ["Lietuvi","Lietuvių","lietuvių"] },
  { code: "nds", name: "Low German", nativeNames: ["Nedderdütsch","Neddersassisch","Nedersaksisch","Plattdeutsch"] },
  { code: "mk", name: "Macedonian", nativeNames: ["Makedonski","македонски"] },
  { code: "ms", name: "Malay", nativeNames: ["Bahasa Melayu"] },
  { code: "ml", name: "Malayalam", nativeNames: ["Malayalani","Malayali","മലയാളം"] },
  { code: "mt", name: "Maltese", nativeNames: ["Malti"] },
  { code: "mr", name: "Marathi", nativeNames: ["Maharashtra","मराठी"] },
  { code: "mn", name: "Mongolian", nativeNames: ["Монгол хэлний бүлэг"] },
  { code: "nv", name: "Navajo", nativeNames: ["Navaho","Diné bizaad"] },
  { code: "nap", name: "Neapolitan", nativeNames: ["Neapolitan-Calabrese"] },
  { code: "nb", name: "Norwegian (Bokmål)", nativeNames: ["norsk bokmål"] },
  { code: "fa", name: "Persian", nativeNames: ["Farsi","فارسی"] },
  { code: "pl", name: "Polish", nativeNames: ["Polnisch","Polski","polski"] },
  { code: "pt", name: "Portuguese", nativeNames: ["Português","português"] },
  { code: "ro", name: "Romanian", nativeNames: ["Daco-Rumanian","Moldavian","Rumanian","română"] },
  { code: "rom", name: "Romany", nativeNames: ["Romani chib"] },
  { code: "ru", name: "Russian", nativeNames: ["Russki","русский"] },
  { code: "sa", name: "Sanskrit", nativeNames: ["संस्कृतम्"] },
  { code: "sr", name: "Serbian", nativeNames: ["Montenegrin","српски"] },
  { code: "sk", name: "Slovak", nativeNames: ["Slovakian","Slovencina","slovenčina"] },
  { code: "sl", name: "Slovenian", nativeNames: ["Slovenscina","slovenščina"] },
  { code: "es", name: "Spanish", nativeNames: ["Castellano","Castilian","Español","español"] },
  { code: "sw", name: "Swahili", nativeNames: ["Kiswahili"] },
  { code: "sv", name: "Swedish", nativeNames: ["Ruotsi","Svenska","svenska"] },
  { code: "tl", name: "Tagalog", nativeNames: ["Tagalog"] },
  { code: "tg", name: "Tajik", nativeNames: ["Galcha","Tadzhik","Tojiki","Тоҷикӣ"] },
  { code: "ta", name: "Tamil", nativeNames: ["Damulian","Tamili","தமிழ்"] },
  { code: "tt", name: "Tatar", nativeNames: ["Tartar","Татар теле"] },
  { code: "te", name: "Telugu", nativeNames: ["Andhra","Telugu","తెలుగు"] },
  { code: "th", name: "Thai", nativeNames: ["Central Tai","Siamese","ไทย"] },
  { code: "tr", name: "Turkish", nativeNames: ["Anatolian","Türkçe"] },
  { code: "tk", name: "Turkmen", nativeNames: ["Trukhmen","Turkmani","Türkmen dili"] },
  { code: "uk", name: "Ukrainian", nativeNames: ["українська"] },
  { code: "ur", name: "Urdu", nativeNames: ["Bihari","اردو"] },
  { code: "uz", name: "Uzbek", nativeNames: ["Oʻzbek tili"] },
  { code: "vi", name: "Vietnamese", nativeNames: ["Annamese","Kinh","Tiếng Việt"] },
  { code: "vo", name: "Volapük", nativeNames: ["Volapük"] },
  { code: "yo", name: "Yoruba", nativeNames: ["Yariba","Yooba","Èdè Yorùbá"] },
  { code: "zu", name: "Zulu", nativeNames: ["Isizulu","Zunda"] }
] as const;

export const LANGUAGE_OPTIONS: LangOption[] = LANGUAGES.map(lang => {
    return {
      value: lang.code,
      label: `${lang.code} ${lang.name} (${lang.nativeNames.join(", ")})`
    }
});
