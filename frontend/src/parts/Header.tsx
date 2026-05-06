
import logo from "../assets/iloader.svg";
import "./Header.css";

function Header() {
    return <header>
        <div className="title-block">
            <img src={logo} alt="iloader" />
            <div>
                <h1>iloader</h1>
                <span className="subtitle">Sideloading Companion</span>
            </div>
        </div>
    </header>
}

export default Header;