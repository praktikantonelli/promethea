import { render } from '@testing-library/react';

import { Button } from './button';

describe('Button', () => {
  it('renders children', () => {
    const { getByRole } = render(<Button>Click me</Button>);

    expect(getByRole('button', { name: 'Click me' })).toBeTruthy();
  });
});
