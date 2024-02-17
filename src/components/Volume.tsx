import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";

function Volume() {
    const [volume, setVolume] = useState(1);

    useEffect(() => {
        changeVolume();
    }, [volume])

    async function changeVolume() {
        await invoke("set_volume", { volume: volume });
    }

    return (
        <input
            type="range"
            min={0}
            max={1}
            step={0.02}
            value={volume}
            onChange={event => {
                setVolume(event.target.valueAsNumber)
            }}
        />
    )
}

export default Volume;