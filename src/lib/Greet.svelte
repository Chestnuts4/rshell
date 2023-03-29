<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit, listen } from "@tauri-apps/api/event";

  let name = "";
  let greetMsg = "";
  let greetMsg2 = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
    // await invoke("first_cmd", { name });
    greetMsg2 = await invoke("first_cmd");

    await invoke("command_name")
      .then((message) => console.log(message))
      .catch((error) => console.error(error));
  }
  async function lis() {
    const unlisten = await listen("click", (event) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
    });
  }
  // emits the `click` event with the object payload
  emit("click", {
    theMessage: "Tauri is awesome!",
  });
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}> Greet </button>
  </div>
  <p>{greetMsg}</p>
  <p>{greetMsg2}</p>
</div>
<div />
