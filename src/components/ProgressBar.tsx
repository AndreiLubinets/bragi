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

    async function seek(time: number) {
        await invoke("seek", { pos: time }).then(() => setCurrentPlaytime());
    }

    function handleClick(event: React.MouseEvent<HTMLProgressElement>) {
        seek(((event.clientX - event.currentTarget.offsetLeft) * length) / event.currentTarget.offsetWidth);
    }

    return (
        <div className="progress">
            <progress value={current} max={length} onClick={handleClick}></progress>
            <span>{convertLength(current)} / {convertLength(length)}</span>
        </div>
    );
};

export default ProgressBar;
