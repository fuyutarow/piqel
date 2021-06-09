import React from 'react';

import { isDevelopment, isStaging, notifyAutoClose as autoClose } from 'plugins/env';

export const debugLog = isDevelopment || isStaging
  ? console.log
  : (...data: Array<any>) => { };

let count = 0;

export const DebugButton: React.FC<{
  onClick?: ((event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void) | undefined;
}> = ({ onClick, children }) => {
  if (!(isDevelopment || isStaging)) return null;
  return (
    <button onClick={onClick}>{children ? children : 'debug'}</button>
  );
};

type Radio = Array<{
  value: any;
  checked: boolean;
}>;

export const DebugRadio: React.FC<{
  radio: Radio;
  setRadio: React.Dispatch<React.SetStateAction<Radio>>;
}> = ({ radio, setRadio }) => {

  const handleRadioClick = (event: React.ChangeEvent<HTMLInputElement>) => {
    const current = radio.map(r => {
      const checked = (r.value.toString() === event.target.value);
      return { ...r, checked };
    });
    setRadio(current);
  };

  const items = radio.map((r) => (
    <label key={r.value.toString()}>
      <input
        type="radio"
        value={r.value}
        checked={r.checked}
        onChange={handleRadioClick}
      />
      {r.value}
    </label>
  ));

  return (<>{items}</>);
};
