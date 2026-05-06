
import { Button } from "@/components/ui/button";
import logo from "../assets/iloader.svg";
import "./Header.css";
import { client } from "@/main";
import { ExternalLinkIcon } from "lucide-react";


function Header() {
    return <header>
        <div className="title-block">
            <img src={logo} alt="iloader" />
            <div>
                <h1>iloader</h1>
                <span className="subtitle">Sideloading Companion</span>
            </div>
        </div>
        <Button variant="ghost" onClick={() => client.openUrl("https://github.com/nab138/iloader-next")}>GitHub <ExternalLinkIcon /></Button>
    </header>
}

export default Header;