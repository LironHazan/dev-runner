import "./sidebar.css";
import {Link} from "react-router-dom";

function Sidebar() {
    return (<>

<nav className="nav-trans">
    <ul className="sidebar-items">
        <li className="sidebar-item">
            <Link to={'runner'}>Runner</Link>
        </li>
        <li className="sidebar-item">
            <Link to={'results-history'}>Results History</Link>
        </li>
    </ul>
</nav>
</>
);
}
export default Sidebar;