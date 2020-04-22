// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React, { useState } from 'react';
import proptypes from 'prop-types';
import crypto from 'crypto';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faTimes } from '@fortawesome/free-solid-svg-icons';
import { Input } from '../Input';

export function EnterPasswordForm({ callbackFn, errorMessage }) {
  const [password, setPassword] = useState(null);
  const [error, setError] = useState(!!errorMessage);
  const [errorMsg, setErrorMsg] = useState(errorMessage);

  const handleChange = event => {
    const { value } = event.target;
    setPassword(value);
  };

  const reset = () => {
    setPassword('');
    setErrorMsg(null);
    setError(false);
  };

  const handleSubmit = event => {
    event.preventDefault();
    const hashedSecret = crypto
      .createHash('sha256')
      .update(password)
      .digest('hex');
    sessionStorage.setItem('KEY_SECRET', hashedSecret);
    reset();
    try {
      callbackFn();
    } catch (err) {
      sessionStorage.removeItem('KEY_SECRET');
      setError(true);
      setErrorMsg(
        `Unable to decrypt key. You may have entered an incorrect password. Error: ${err.message}`
      );
    }
  };

  return (
    <div className="wrapper">
      {!error && (
        <>
          <h2>Enter your password</h2>
          <form id="enter-password-form" aria-label="enter-password-form">
            <Input
              type="password"
              label="Password"
              id="password"
              name="password"
              onChange={handleChange}
              value={password}
            />
            <button
              className="submit"
              onClick={handleSubmit}
              disabled={!password}
            >
              Submit
            </button>
          </form>
        </>
      )}
      {error && (
        <div className="error-wrapper">
          <FontAwesomeIcon className="icon" icon={faTimes} />
          <div className="error">
            <span>{errorMsg}</span>
          </div>
          <div className="actions-wrapper">
            <button onClick={reset}>Reset</button>
          </div>
        </div>
      )}
    </div>
  );
}

EnterPasswordForm.propTypes = {
  callbackFn: proptypes.func.isRequired,
  errorMessage: proptypes.string
};

EnterPasswordForm.defaultProps = {
  errorMessage: null
};
