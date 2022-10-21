<script>
  import BottomPlayer from './lib/BottomPlayer.svelte';
  import FileContainer from './lib/FileContainer.svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let DEF_PATH = "!";
  let MUSIC_ARRAY = {songs: []};


  invoke("get_os_dir").then(
    (path) => {
      DEF_PATH = path;
      get_music();
    }
  );

  async function get_music() {
    MUSIC_ARRAY = JSON.parse(await invoke("scan_dir", {path: DEF_PATH}));
    console.table(MUSIC_ARRAY.songs);
  }

</script>

<main class="container">
  <FileContainer songs={MUSIC_ARRAY.songs}/>
  <BottomPlayer />
</main>

<style>

</style>
