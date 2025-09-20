import Select, {ActionMeta} from "react-select";
import {LangOption, LANGUAGE_OPTIONS} from "./lib/languages.ts";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";



function App() {
  const [fromLang, setFromLang] = useState<LangOption | null>(null);
  const [toLang, setToLang] = useState<LangOption | null>(null);

  const handleFromLangChange = (input: LangOption | null, _actionMeta: ActionMeta<LangOption>) => {
    const option = input ?? LANGUAGE_OPTIONS[0];
    invoke("set_from_lang", {lang: option.value}).then(()=>{
      setFromLang(option)
    })
  }
  const handleToLangChange = (input: LangOption | null, _actionMeta: ActionMeta<LangOption>) => {
    const option = input ?? LANGUAGE_OPTIONS[0];
    invoke("set_to_lang", {lang: option.value}).then(()=>{
      setToLang(option)
    })
  }

  useEffect(() => {
    invoke<string>("get_from_lang").then((value) => {
      const option = LANGUAGE_OPTIONS.find((item) => item.value == value);
      setFromLang(option!)
    }).catch((e) => {
      alert(e)
    });

    invoke<string>("get_to_lang").then((value) => {
      const option = LANGUAGE_OPTIONS.find((item) => item.value == value);
      setToLang(option!)
    }).catch((e) => {
      alert(e)
    })
  }, []);

  return (
    <main className="container">
      <div className={"flex flex-row"}>
        <div className={"w-1/2"}>
          <label>Translate From</label>
          <Select
            value={fromLang}
            isSearchable={true}
            onChange={handleFromLangChange}
            options={LANGUAGE_OPTIONS}
          >
          </Select>
        </div>
        <div className={"w-1/2"}>
          <label>Translate To</label>
          <Select
            value={toLang}
            isSearchable={true}
            onChange={handleToLangChange}
            options={LANGUAGE_OPTIONS}
          >
          </Select>
        </div>
      </div>

    </main>
  );
}

export default App;
