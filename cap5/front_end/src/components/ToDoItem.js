import React from 'react';

export const ToDoItem = ({ title, status, send_request }) => {
  const process_status = status === 'PENDING' ? 'edit' : 'delete';

  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
      <p>{title}</p>
      <button
        onClick={() => {
          send_request(title, status, process_status);
        }}
      >
        {process_status}
      </button>
    </div>
  );
};
