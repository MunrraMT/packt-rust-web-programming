import React, { useState } from 'react';

export const CreateToDoItem = ({ create_request }) => {
  const [title, set_title] = useState('');

  const handle_change = (e) => {
    set_title(e.target.value);
  };

  const handle_submit = (e) => {
    e.preventDefault();
    create_request(title);
  };

  return (
    <form onSubmit={handle_submit} className="input-container">
      <input
        type="text"
        id="name"
        onChange={handle_change}
        value={title}
        placeholder="create to do item"
      />

      <button type="submit" className="action-button">
        Create
      </button>
    </form>
  );
};
