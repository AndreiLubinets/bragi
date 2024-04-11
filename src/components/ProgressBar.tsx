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
        console.log(time);
        //await invoke("seek", { pos: time });
    }

    function handleClick(event: React.MouseEvent<HTMLProgressElement>) {
        seek((event.clientX * length) / event.currentTarget.offsetWidth);
    }

    return (
        <div className="progress">
            <input type="range" value={current} max={length} onClick={handleClick}></input>
            <span>{convertLength(current)} / {convertLength(length)}</span>
        </div>
    );
};

export default ProgressBar;
