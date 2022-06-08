let animationId;
const playPauseButton = document.getElementById('play-pause');

export function play() {
    animationId = requestAnimationFrame(renderLoop);
    playPauseButton.textContent = "Pause";
}

function pause() {
    cancelAnimationFrame(animationId);
    playPauseButton.textContent = "Play";
    animationId = null;
}

playPauseButton.addEventListener('click', () => {
    // cancel animation events to "pause"
    if (animationId) {
        pause();
    } else {
        play();
    }
})
