// Convexe Hülle
// (c) Georg Umlauf, 2015

#include "glwidget.h"
#include <QtGui>
#include <GL/glu.h>
#include "mainwindow.h"


GLWidget::GLWidget(QWidget *parent) : QGLWidget(parent)
{	
}

GLWidget::~GLWidget()
{
}

void GLWidget::paintGL()
{
    // clear
    glClear(GL_COLOR_BUFFER_BIT);

    // Koordinatensystem
    glColor3f(0.5,0.5,0.5);
    glBegin(GL_LINES);
    glVertex2f(-1.0, 0.0);
    glVertex2f( 1.0, 0.0);
    glVertex2f( 0.0,-1.0);
    glVertex2f( 0.0, 1.0);
    glEnd();

	// Konvexe Hülle zeichnen
	// TODO: draw convex hull using glBegin(GL_LINE_STRIP); ... glEnd();
}


void GLWidget::initializeGL()
{
    resizeGL(width(),height());
}

void GLWidget::resizeGL(int width, int height)
{
    aspectx=1.0;
    aspecty=1.0;
    if (width>height) aspectx = float(width) /height;
    else              aspecty = float(height)/ width;
    glViewport    (0,0,width,height);
    glMatrixMode  (GL_PROJECTION);
    glLoadIdentity();
    gluOrtho2D    (-aspectx,aspectx,-aspecty,aspecty);
    glMatrixMode  (GL_MODELVIEW);
    glLoadIdentity();
}

QPointF GLWidget::transformPosition(QPoint p)
{
    return QPointF( (2.0*p.x()/ width() - 1.0)*aspectx,
		           -(2.0*p.y()/height() - 1.0)*aspecty);
}

void GLWidget::keyPressEvent(QKeyEvent * event)
{
	switch (event->key()) {
	default:
		break;
	}
	update();
}

void GLWidget::mousePressEvent(QMouseEvent *event)
{
	QPointF posF = transformPosition(event->pos());
	if (event->buttons() & Qt::LeftButton ) {
		// TODO: add point to point list and recompute convex hull
	}
    update(); 
}


void GLWidget::radioButton1Clicked()
{
	// TODO: toggle to Jarvis' march
    update();
}

void GLWidget::radioButton2Clicked()
{
	// TODO: toggle to Graham's scan
    update();
}
