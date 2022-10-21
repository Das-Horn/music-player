export class Player {
    constructor(player) {
        this.player = player;
        this.playlistQueue = [];
        this.currentSong = String;
    }

    async changePlayerSource(src) {
        // FIXME : Implement Error Catching here
        this.player.src = src;
    }

    async nextSong() {
       // TODO : Implement Next Song 
    }

    async togglePlay() {
        // TODO : Implement TogglePlay
    }

    async seekBarLoop() {
        // TODO : Implement SeekBar code stuff
    }
}