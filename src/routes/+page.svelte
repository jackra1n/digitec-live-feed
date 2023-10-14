<script>
    export let data;

    let socialShopping = data.data[0].data.socialShopping;
    let live_feed_items = socialShopping.items;
    console.log(live_feed_items);
    
</script>

<div class="h-screen flex justify-center items-center">
    <div class="border-2 border-sky-200 p-2">
        <h1 class="text-2xl">Itmes:</h1>
        <div>
            <p>
                {live_feed_items}
            </p>

        </div>

    </div>
</div>

