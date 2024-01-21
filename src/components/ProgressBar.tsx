import { useState } from "react";
import "./ProgressBar.css";

const ProgressBar = ({ length }: { length: string }) => {
    const [current, setCurrent] = useState(0);

    return (
        <div className="progress">
            <progress value={current} max={length}></progress>
        </div>
    );
};

export default ProgressBar;