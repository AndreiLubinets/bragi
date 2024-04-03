import ITrack from "../interfaces/track";
import "./TrackInfo.css";
import default_cover from '../assets/default-cover.svg';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

const TrackInfo = ({ track }: { track: ITrack | undefined }) => {

    const [albumCover, setAlbumCover] = useState('');

    useEffect(() => {
        const getAlbumCover = async () => {
            try {
                const buffer = new Uint8Array(await invoke("get_album_cover", {}));
                const blob = new Blob([buffer], { type: 'image/jpg' });
                const url = URL.createObjectURL(blob);
                setAlbumCover(url);

                return () => {
                    URL.revokeObjectURL(albumCover);
                };
            } catch (err) {
                setAlbumCover('');
            }
        }
        getAlbumCover();

    }, [track])



    return (
        <>
            {track &&
                <div className="trackinfo">
                    <img src={albumCover !== '' ? albumCover : default_cover}></img>
                    <div>
                        <h2>{track.title}</h2>
                        <h5>{track.artist}</h5>
                        <h5>{track.album}</h5>
                    </div>
                </div >
            }
        </>
    );
};

export default TrackInfo;
