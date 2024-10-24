async function fetchApiData() {
  const orderID = document.getElementById("orderID").value;
  const invoke = window.__TAURI__.invoke;

  try {
    const result = await invoke("get_label", {
      orderId: orderID,
    });

    // Display the fetched API data momentarily
    document.getElementById("api-data").innerText = JSON.stringify(
      result,
      null,
      2
    );
  } catch (error) {
    showToast(error);
  }
}

function showToast(message) {
  const toast = document.createElement("div");
  toast.className = "toast";
  toast.textContent = message;
  document.body.appendChild(toast);

  setTimeout(() => {
    toast.remove();
  }, 4000);
}
