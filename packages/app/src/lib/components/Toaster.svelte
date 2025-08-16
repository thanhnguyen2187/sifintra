<script lang="ts">
let messageInfo: string = $state("");
let messageSuccess: string = $state("");
let messageError: string = $state("");
let messageWarning: string = $state("");

export function setMessageInfo(message: string, timeoutMs = 1_500) {
  messageInfo = message;
  setTimeout(() => {
    messageInfo = "";
  }, timeoutMs);
}

export function setMessageSuccess(message: string, timeoutMs = 1_500) {
  messageSuccess = message;
  setTimeout(() => {
    messageSuccess = "";
  }, timeoutMs);
}

export function setMessageWarning(message: string, timeoutMs = 1_500) {
  messageWarning = message;
  setTimeout(() => {
    messageWarning = "";
  }, timeoutMs);
}

export function setMessageError(message: string, timeoutMs = 1_500) {
  messageError = message;
  setTimeout(() => {
    messageError = "";
  }, timeoutMs);
}
</script>

<div class="toast toast-center transition-all">
    {#if messageInfo !== ""}
        <div class="alert alert-info">
            <span>{messageInfo}</span>
        </div>
    {/if}
    {#if messageSuccess !== ""}
        <div class="alert alert-success">
            <span>{messageSuccess}</span>
        </div>
    {/if}
    {#if messageWarning !== ""}
        <div class="alert alert-warning">
            <span>{messageWarning}</span>
        </div>
    {/if}
    {#if messageError !== ""}
        <div class="alert alert-error">
            <span>{messageError}</span>
        </div>
    {/if}
</div>