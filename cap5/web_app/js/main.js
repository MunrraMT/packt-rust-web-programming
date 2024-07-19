function render_items(items, process_type, element_id, process_function) {
  const container = document.createElement('div');
  const items_meta = [];

  container.innerHTML = items.forEach((item) => {
    const id = process_type + '-' + title;
    const div = document.createElement('div');
    const text = document.createElement('p');
    const btn = document.createElement('button');

    text.textContent = item.title;
    btn.id = id;
    btn.textContent = 'edit';
    btn.addEventListener('click', process_function);

    items_meta.push({
      id,
      title: item.title.replace(/ /g, '-'),
    });

    div.appendChild(text);
    div.appendChild(btn);
    container.appendChild(div);
  });

  document.querySelector(`#${element_id}`).appendChild(container);
}
