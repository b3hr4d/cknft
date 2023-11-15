import React from "react";

/**
 *
 * @param header_text - main popup text
 * @param body_text - body popup text
 * 
 */
function Popup({header_text, body_text}) {

  return (
    <div className="popup">
      <h1></h1>
      <h2>{header_text}</h2>
      <p>{body_text}</p>
    </div>
  );
}

export default Popup;
