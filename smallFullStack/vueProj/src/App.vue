<script setup>

</script>

<template>
    <div class="navbar bg-base-100">
        <div class="flex-none py-2 ms-5">
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
    <footer class="footer p-10 bg-neutral text-neutral-content">

    </footer>
</template>

<style scoped>

</style>
