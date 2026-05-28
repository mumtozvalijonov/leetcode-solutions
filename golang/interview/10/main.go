// Package challenge10 contains the solution for Challenge 10.
package challenge10

import (
	"fmt"
	"math"
	"slices"
	// Add any necessary imports here
)

// Shape interface defines methods that all shapes must implement
type Shape interface {
	Area() float64
	Perimeter() float64
	fmt.Stringer // Includes String() string method
}

// Rectangle represents a four-sided shape with perpendicular sides
type Rectangle struct {
	Width  float64
	Height float64
}

func NewRectangle(width, height float64) (*Rectangle, error) {
	if width <= 0 || height <= 0 {
		return nil, fmt.Errorf("Invalid rectangle sides")
	}
	return &Rectangle{Width: width, Height: height}, nil
}

func (r *Rectangle) Area() float64 {
	return r.Width * r.Height
}

func (r *Rectangle) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

func (r *Rectangle) String() string {
	return fmt.Sprintf("Rectangle with width=%f & height=%f", r.Width, r.Height)
}

// Circle represents a perfectly round shape
type Circle struct {
	Radius float64
}

func NewCircle(radius float64) (*Circle, error) {
	if radius <= 0 {
		return nil, fmt.Errorf("Invalid circle radius")
	}
	return &Circle{Radius: radius}, nil
}

func (c *Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

func (c *Circle) Perimeter() float64 {
	return 2 * math.Pi * c.Radius
}

func (c *Circle) String() string {
	return fmt.Sprintf("Circle with radius %f", c.Radius)
}

type Triangle struct {
	SideA float64
	SideB float64
	SideC float64
}

func NewTriangle(a, b, c float64) (*Triangle, error) {
	if a <= 0 || b <= 0 || c <= 0 {
		return nil, fmt.Errorf("Invalid triangle sides")
	}

	if (a+b) <= c || (b+c) <= a || (a+c) <= b {
		return nil, fmt.Errorf("Invalid triangle sides")
	}
	return &Triangle{SideA: a, SideB: b, SideC: c}, nil
}

func (t *Triangle) Area() float64 {
	s := t.Perimeter() / 2
	return math.Sqrt(s * (s - t.SideA) * (s - t.SideB) * (s - t.SideC))
}

func (t *Triangle) Perimeter() float64 {
	return t.SideA + t.SideB + t.SideC
}

func (t *Triangle) String() string {
	return fmt.Sprintf("Triangle with sides %f, %f, and %f", t.SideA, t.SideB, t.SideC)
}

type ShapeCalculator struct{}

func NewShapeCalculator() *ShapeCalculator {
	return &ShapeCalculator{}
}

// PrintProperties prints the properties of a shape
func (sc *ShapeCalculator) PrintProperties(s Shape) {
	fmt.Println(s)
	fmt.Printf("Perimeter: %f\n", s.Perimeter())
	fmt.Printf("Area: %f\n", s.Area())
}

// TotalArea calculates the sum of areas of all shapes
func (sc *ShapeCalculator) TotalArea(shapes []Shape) float64 {
	result := 0.0
	for _, s := range shapes {
		result += s.Area()
	}
	return result
}

func (sc *ShapeCalculator) LargestShape(shapes []Shape) Shape {
	if len(shapes) == 0 {
		return nil
	}

	result := shapes[0]
	maxArea := result.Area()

	for _, s := range shapes[1:] {
		curArea := s.Area()
		if curArea > maxArea {
			maxArea = curArea
			result = s
		}
	}

	return result
}

func (sc *ShapeCalculator) SortByArea(shapes []Shape, ascending bool) []Shape {
	slices.SortFunc(shapes, func(a, b Shape) int {
		areaA := a.Area()
		areaB := b.Area()

		switch {
		case areaA < areaB:
			if ascending {
				return -1
			}
			return 1
		case areaA > areaB:
			if ascending {
				return 1
			}
			return -1
		default:
			return 0
		}
	})

	return shapes
}
