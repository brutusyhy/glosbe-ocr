import React from "react";
import ReactDOM from "react-dom/client";
import { Routes, Route, BrowserRouter} from "react-router-dom";
import App from "./App";
import SearchResult from "./SearchResult.tsx";
import Overlay from "./Overlay.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
        <Routes>
            <Route path="/" element={<App />}/>
            <Route path="/search-result" element={<SearchResult />}/>
            <Route path="/overlay" element={<Overlay />} />
        </Routes>
    </BrowserRouter>
  </React.StrictMode>,
);

