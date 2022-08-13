/**
 * Cell type of any color, size, and placement
 * @author Benedikt Arnarsson
 */

// Import React
import React from 'react';

// Import shared types
import Color from '../types/Color';


/*------------------------------------------------------------------------*/
/*                                  Types                                 */
/*------------------------------------------------------------------------*/

// Props definition
type Props = {
  // Color of the cell
  color: Color,
  // Size the cell 
  cellSize: number,
  // Position on x-axis, a.k.a. column number
  x: number,
  // Position on y-axis, a.k.a. row number
  y: number,
};


/*------------------------------------------------------------------------*/
/*                                Component                               */
/*------------------------------------------------------------------------*/

const Cell: React.FC<Props> = (props) => {
  /*------------------------------------------------------------------------*/
  /*                                  Setup                                 */
  /*------------------------------------------------------------------------*/

  /* -------------- Props ------------- */

  // Destructure all props
  const {
    color,
    cellSize,
    x,
    y,
  } = props;

  /*------------------------------------------------------------------------*/
  /*                                 Render                                 */
  /*------------------------------------------------------------------------*/

  /*----------------------------------------*/
  /*                 Main UI                */
  /*----------------------------------------*/

  return (
    <div
      className="Cell"
      style={{
        position: 'absolute',
        background: color,
        left: `${cellSize * x + 1}px`,
        top: `${cellSize * y + 1}px`,
        width: `${cellSize - 1}px`,
        height: `${cellSize - 1}px`,
      }}
    />
  );
};

/*------------------------------------------------------------------------*/
/*                                 Wrap Up                                */
/*------------------------------------------------------------------------*/

// Export component
export default Cell;
