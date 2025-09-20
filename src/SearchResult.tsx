import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";

function SearchResult() {
    const [payload, setPayload] = useState('');
    useEffect(() => {
        const unlisten = listen<string>("search-result", (event) => {
            console.log(event);
            console.log(event.payload);
            setPayload(event.payload);
        });
        console.log("success")
        return () => {
            unlisten.then(un => un())
        };
    }, [])

    return <div id="content" dangerouslySetInnerHTML={{ __html: payload }} />

}

export default SearchResult;