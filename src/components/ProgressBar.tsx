import { useState } from "react";
import "./ProgressBar.css";
import { invoke } from "@tauri-apps/api";
import { useInterval } from "../util/poll";
import { convertLength } from "../util/time";

const ProgressBar = ({ length }: { length: number }) => {
    const [current, setCurrent] = useState(0);

    useInterval(() => {
        setCurrentPlaytime();
    }, 1000);

    async function setCurrentPlaytime() {
        setCurrent(await invoke("playtime", {}));
    }

    return (
        <div className="progress">
            <progress value={current} max={length}></progress>
            <span>{convertLength(current)} / {convertLength(length)}</span>
        </div>
    );
};

export default ProgressBar;
