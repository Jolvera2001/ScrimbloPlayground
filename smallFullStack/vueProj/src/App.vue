<script setup>
import ButtonCounter from './components/CounterButton.vue'
</script>

<template>
    <div class="navbar bg-base-100">
        <div class="flex-none ms-5">
            <div class="drawer">
                <input id="my-drawer" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content">
                    <label for="my-drawer" class="btn btn-primary drawer-button">Open Drawer</label>
                </div>
                <div class="drawer-side">
                    <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                    <ul class="menu p-4 w-60 min-h-full bg-base-200 text-base-content">
                        <li><a>Item1</a></li>
                        <li><a>Item2</a></li>
                    </ul>
                </div>
            </div>
        </div>
        <div class="flex-1 ms-5">
            <h2>Welcome!</h2>
        </div>
    </div>
    <body>
        <div class="md:container md:mx-auto ps-10 py-2">
            <h1>Dashboard</h1>
            <ButtonCounter word="Scrimblo" />
        </div>
    </body>
    <footer class="footer p-10 bg-neutral text-neutral-content my-5">
        <a>Placeholder</a>
    </footer>
</template>

<style scoped>

</style>
