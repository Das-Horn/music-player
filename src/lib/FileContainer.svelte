<script>
    import SongItem from "./SongItem.svelte";
    export let songs = [];


</script>

<main>
    <div class="cont">
        {#each songs as song}
            <SongItem song={song}/>
        {/each}
    </div>
</main>

<style scoped>
    .cont {
        background-color: var(--base-sec);
        position: absolute;
        height: calc(100vh - 40px);
        width:80vw;
        top:0;
        right:0;
        border-left: solid 2px #fff;
        overflow-x: auto;
    }    
</style>