<script>
    export let song = {}  
    import { readBinaryFile } from '@tauri-apps/api/fs';
    // Code to Generate song name from path
    let songName = song.path.split("/");
    songName = songName[songName.length - 1].split(".");
    songName.pop();

    songName = songName.join(".");

    async function set_source() {
        const audioCtx = new AudioContext();
        const x = document.querySelector("#player");
        // ^ Variables
        // Read File
        let audioFile = new Blob([await readBinaryFile(song.path)], {type : 'audio/mp3  '});
        //set source

        x.src = URL.createObjectURL(audioFile);
        console.info("Now Playing : "+song.path);
    }
</script>

<main>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div on:click={set_source} class="song-card">
        <p>{songName}</p>
    </div>
</main>

<style>
    .song-card {
        display:flex;
        width: 95%;
        padding:5px;
        height: max-content;
        background-color: rgb(77, 0, 91);
        color: #fff;
        border-radius:10px;
        margin : 4px auto;
        cursor: pointer;
    }
</style>