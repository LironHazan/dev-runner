import { Outlet } from 'react-router-dom';
import Sidebar from "./components/Sidebar/Sidebar";
import "./app.css";

function App() {
  return (
    <>
        <div id="sidebar">
            <Sidebar />
        </div>
        <div id="detail">
            <Outlet />
        </div>
    </>
  );
}

export default App;
