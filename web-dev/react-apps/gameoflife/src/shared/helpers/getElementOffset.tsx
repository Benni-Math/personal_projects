import React from 'react';

// ISSUE: this does not work -- I need a ref, but I'm using functional components.

/**
 * Finds the offset of a (x, y) location on the stream
 * @author Benedikt Arnarsson
 * @param grid React.ReactNode of the grid which we are using
 */
const getElementOffset = (grid: React.ReactNode): { x: number, y: number } => {
  const rect = grid.getBoundingClientRect();

  return {
    x: 0,
    y: 0,
  }
};

export default getElementOffset;
