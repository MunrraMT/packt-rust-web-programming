import React, { useState } from 'react';

export const CreateToDoItem = ({ create_request }) => {
  const [title, set_title] = useState('');

  const handle_change = (e) => {
    set_title(e.target.value);
  };

  const handle_click = () => {
    create_request(title);
  };

  return (
    <div className="input-container">
      <input
        type="text"
        id="name"
        onChange={handle_change}
        value={title}
        placeholder="create to do item"
      />

      <button onClick={handle_click} className="action-button">
        Create
      </button>
    </div>
  );
};
