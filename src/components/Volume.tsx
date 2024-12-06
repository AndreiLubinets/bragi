import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./Volume.css";
import volume_max_icon from '../assets/volume-max.svg';
import volume_min_icon from '../assets/volume-min.svg';
import volume_mute_icon from '../assets/volume-mute.svg';
import { listen } from "@tauri-apps/api/event";

function Volume() {
    const [volume, setVolume] = useState(1);
    const [visible, setVisible] = useState(false);

    useEffect(() => {
        changeVolume();
    }, [volume])

    listen('volume_updated', async () => {
        setVolume(await getVolume());
    })

    async function getVolume(): Promise<number> {
        return await invoke("get_volume", {});
    }

    async function changeVolume() {
        await invoke("set_volume", { volume: volume });
    }

    function selectIcon(): string {
        if (volume > 0.5) {
            return volume_max_icon;
        } else if (volume > 0) {
            return volume_min_icon;
        } else {
            return volume_mute_icon;
        }
    }

    return (
        <div className="volume">
            <button
                className="control-button"
                onClick={() => setVisible(prev => !prev)}
            >
                <img src={selectIcon()} />
            </button>
            <input
                className={visible ? "visible" : ""}
                type="range"
                min={0}
                max={1}
                step={0.02}
                value={volume}
                onChange={event => {
                    setVolume(event.target.valueAsNumber)
                }}
            />
        </div>
    )
}

export default Volume;
