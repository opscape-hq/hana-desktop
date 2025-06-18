import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import "./fonts.css";
import DragBar from "./components/DragBar";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <DragBar />
    <App />
  </React.StrictMode>,
);
