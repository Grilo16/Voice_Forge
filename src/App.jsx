import { BrowserRouter as Router, Routes, Route } from "react-router-dom"
import { addPage, makeRoutes, pages } from "./routeRegistry/routeRegistry";
import { AppLayout } from "./components";
import * as page from "./pages";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";

function App() {
  addPage(<page.HomePage/>, "Home", "/", true)
  addPage(<page.PuffinPage/>, "Puffin", "/puffin", true)
  addPage(<page.BuildPage/>, "Build", "/build")
  addPage(<page.RepairPage/>, "Repair", "/repair")
  addPage(<page.BlendPage/>, "Blend", "/blend")
  const routes = makeRoutes(pages)

  const [cwd, setCwd] = useState("");
  async function getCwd() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setCwd(await invoke("get_cwd"));
  }
  useEffect(() => {
    getCwd()
  }, [])
  
  return (
    <Router>
      <AppLayout pages={pages}>
       <Routes>{routes}</Routes>
      </AppLayout>
    </Router>
  );
}

export default App;
