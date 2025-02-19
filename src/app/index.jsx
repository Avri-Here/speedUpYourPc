
import "./style.css";

import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import cleanTempFiles from "../assets/img/gif/clean.gif";

export default () => {

  const [name, setName] = useState("");
  const [greetMsg, setGreetMsg] = useState("");

  const cleanTempFolders = async () => {

    //  https://tauri.app/develop/calling-rust/

    setGreetMsg("Cleaning Temp Folders in Progress...");
    await invoke("clean_temp_folders").then((response) => {
      // print(response)
      setGreetMsg(response);
    });
  }

  return (

    <main className="container">
      <h1> Start here to speed up your PC </h1>

      <div className="row">

        <img src={cleanTempFiles} className="cleanTempFiles" onClick={async () => {

          await cleanTempFolders();
        }} />

      </div>
      <p>{greetMsg}</p>
    </main>
  );

}


