import { useState } from "react";
import "./ProgressBar.css";
import { invoke } from "@tauri-apps/api/core";
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

    function handleChange(event: React.ChangeEvent<HTMLInputElement>) {
        seek(event.target.valueAsNumber);
    }

    return (
        <div className="progress">
            <input type="range" value={current} max={length} onChange={handleChange}></input>
            <span>{convertLength(current)} / {convertLength(length)}</span>
        </div>
    );
};

export default ProgressBar;
