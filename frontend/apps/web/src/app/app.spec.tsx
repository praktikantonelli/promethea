import { render } from '@testing-library/react';

import App from './app';

describe('App', () => {
  it('renders the shared UI package button', () => {
    const { getByRole, getByText } = render(<App />);

    expect(getByText('Promethea frontend')).toBeTruthy();
    expect(getByRole('button', { name: 'Shared UI button' })).toBeTruthy();
  });
});
