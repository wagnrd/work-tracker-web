<script lang="ts">
</script>

<div class="container">
    <div class="w-container">
        <div class="w-background">W</div>
        W
    </div>
    T
</div>

<style>
    .container {
        width: 50px;
        height: 50px;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: black;
        color: white;
        font-size: 20px;
    }

    .w-container {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1;
    }

    .w-background {
        position: absolute;
        color: black;
        font-size: 1.3em;
        font-weight: bold;
        z-index: -1;
    }
</style>
