import { useEffect, useRef, useState } from "react";

class AnimationManager {
    private _animation: number | null = null;
    private callback: () => void;
    private lastFrame = -1;
    private frameTime = 1000 / 30;

    constructor(callback: () => void, fps = 30) {
        this.callback = callback;
        this.frameTime = 1000 / fps;
    }

    updateFPS(fps: number) {
        this.frameTime = 1000 / fps;
    }

    start() {
        if (this._animation != null) return;
        this._animation = requestAnimationFrame(this.update);
    }

    pause() {
        if (this._animation == null) return;
        this.lastFrame = -1;
        cancelAnimationFrame(this._animation);
        this._animation = null;
    }

    private update = (time: number) => {
        const { lastFrame } = this;
        let delta = time - lastFrame;
        if (this.lastFrame === -1) {
            this.lastFrame = time;
        } else {
            while (delta >= this.frameTime) {
                this.callback();
                delta -= this.frameTime;
                this.lastFrame += this.frameTime;
            }
        }
        this._animation = requestAnimationFrame(this.update);
    };
}

export default function ASCIIAnimation({
    className = "",
    fps = 24,
    frameCount = 60,
    frameFolder = "frames",
}) {
    const [frames, setFrames] = useState<string[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [currentFrame, setCurrentFrame] = useState(0);
    const framesRef = useRef<string[]>([]);

    const [animationManager] = useState(
        () =>
            new AnimationManager(() => {
                setCurrentFrame((current) => {
                    if (framesRef.current.length === 0) return current;
                    return (current + 1) % framesRef.current.length;
                });
            }, fps),
    );

    useEffect(() => {
        const loadFrames = async () => {
            try {
                const frameFiles = Array.from(
                    { length: frameCount },
                    (_, i) => `frame_${String(i + 1).padStart(4, "0")}.txt`,
                );

                const framePromises = frameFiles.map(async (filename) => {
                    const response = await fetch(`/${frameFolder}/${filename}`);
                    if (!response.ok) {
                        throw new Error(`Failed to fetch ${filename}: ${response.status}`);
                    }
                    return await response.text();
                });

                const loadedFrames = await Promise.all(framePromises);
                setFrames(loadedFrames);
                framesRef.current = loadedFrames;
                setCurrentFrame(0);
            } catch (error) {
                console.error("Failed to load ASCII frames:", error);
            } finally {
                setIsLoading(false);
            }
        };

        loadFrames();
    }, [frameCount, frameFolder]);

    useEffect(() => {
        animationManager.updateFPS(fps);
    }, [fps, animationManager]);

    useEffect(() => {
        if (frames.length === 0) return;

        const reducedMotion =
            window.matchMedia(`(prefers-reduced-motion: reduce)`).matches === true;

        if (reducedMotion) {
            return;
        }

        const handleFocus = () => animationManager.start();
        const handleBlur = () => animationManager.pause();

        window.addEventListener("focus", handleFocus);
        window.addEventListener("blur", handleBlur);

        if (document.visibilityState === "visible") {
            animationManager.start();
        }

        return () => {
            window.removeEventListener("focus", handleFocus);
            window.removeEventListener("blur", handleBlur);
            animationManager.pause();
        };
    }, [animationManager, frames.length]);

    if (isLoading) {
        return (
            <div className={`font-mono whitespace-pre overflow-hidden ${className}`}>
                Loading ASCII animation...
            </div>
        );
    }

    if (!frames.length) {
        return (
            <div className={`font-mono whitespace-pre overflow-hidden ${className}`}>
                No frames loaded
            </div>
        );
    }

    return (
        <div
            className={`relative font-mono whitespace-pre overflow-hidden text-[0.25rem] leading-none ${className}`}
        >
            <div className="relative">
                {frames[currentFrame]}
            </div>
        </div>
    );
}
