#include <jpeglib.h>
#include <stdio.h>
#include <stdlib.h>

//extern JSAMPLE * image_buffer;
int main() {

	/*********** Decompressing and reading **************/
	
	struct jpeg_decompress_struct cinfo1;
	struct jpeg_error_mgr jerr1;
	cinfo1.err = jpeg_std_error(&jerr1);
	jpeg_create_decompress(&cinfo1);

	FILE * infile;
	if ((infile = fopen("testimage.jpeg", "rb")) == NULL) {
		fprintf(stderr, "Can't open %s\n","testimage.jpeg" );
		return 0;
	}
	jpeg_stdio_src(&cinfo1, infile);

	jpeg_read_header(&cinfo1, TRUE);
	jpeg_start_decompress(&cinfo1);

	int row_stride1;
	row_stride1 = cinfo1.output_width * cinfo1.output_components;

	printf("Image width: %i\n", cinfo1.output_width);
	printf("Image height: %i\n", cinfo1.output_height);
	printf("Image color components per pixel: %i\n", cinfo1.out_color_components);	// 3 = RGB, 1 = Grayscale
	printf("Color space: %d\n", cinfo1.jpeg_color_space);

	int width = cinfo1.output_width;
	int height = cinfo1.output_height;
	int components = cinfo1.out_color_components; // 3 = RGB, 1 = Grayscale


	int bitmap_size = cinfo1.output_width * cinfo1.output_height * cinfo1.out_color_components;
	unsigned char * image_buffer = (unsigned char *)malloc(bitmap_size);
	unsigned char * row_pointer1[1];
	row_pointer1[0] = (unsigned char *)malloc(cinfo1.output_width * cinfo1.out_color_components);

	unsigned int location = 0;
	while (cinfo1.output_scanline < cinfo1.output_height) {
		jpeg_read_scanlines(&cinfo1, row_pointer1, 1);
		for (int i=0; i<cinfo1.image_width * cinfo1.out_color_components; i++) {
			image_buffer[location++] = row_pointer1[0][i];
		}

	}
	jpeg_finish_decompress(&cinfo1);
	jpeg_destroy_decompress(&cinfo1);
	//free(row_pointer1[0]);
	fclose(infile);



	/*********** Compressing and writing **************/
	struct jpeg_compress_struct cinfo2;
	struct jpeg_error_mgr jerr2;
	cinfo2.err = jpeg_std_error(&jerr2);
	jpeg_create_compress(&cinfo2);

	FILE * outfile;
	if ((outfile = fopen("output.jpeg", "wb")) == NULL) {
		fprintf(stderr, "Can't open %s\n","output.jpeg" );
		return 0;
	}
	jpeg_stdio_dest(&cinfo2, outfile);

	// Require info before writing
	cinfo2.image_width = width;
	cinfo2.image_height = height;
	cinfo2.input_components = components;	// 3 = RGB, 1 = Grayscale
	cinfo2.in_color_space = JCS_RGB;		//JCS_RGB or JCS_GRAYSCALE
	jpeg_set_defaults(&cinfo2);

	// Optional parameter settings 
	// int quality = 100; // 1-100
	// jpeg_set_quality(&cinfo2, quality, TRUE);

	jpeg_start_compress(&cinfo2, TRUE);

	unsigned char * row_pointer[1];
	int row_stride;
	row_stride = width * 3; // JSAMPLEs per row in image (1 if grayscale?)
	
	while(cinfo2.next_scanline < cinfo2.image_height) {
		row_pointer[0] = & image_buffer[cinfo2.next_scanline * row_stride];
		jpeg_write_scanlines(&cinfo2, row_pointer, 1);
	}
	jpeg_finish_compress(&cinfo2);
	fclose(outfile);

	jpeg_destroy_compress(&cinfo2);


	return 0;
}
