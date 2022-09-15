import "./sidebar.css";

function Sidebar() {
    return (<>

<nav className="nav-trans">
    <ul className="sidebar-items">
        <li className="sidebar-item">
            <a href={'runner'}>Runner</a>
        </li>
        <li className="sidebar-item">
            <a href={'results-history'}>Results History</a>
        </li>
        <li className="sidebar-item">
            <a href={'p3'}>Page 3</a>
        </li>
    </ul>
</nav>
</>
);
}
export default Sidebar;