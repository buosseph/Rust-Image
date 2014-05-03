#include <jpeglib.h>
#include <stdio.h>

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
	
	unsigned char ** buffer = (*cinfo1.mem->alloc_sarray)
		((j_common_ptr) &cinfo1, JPOOL_IMAGE, row_stride1, 1);


	printf("Image width: %i\n", cinfo1.output_width);
	printf("Image height: %i\n", cinfo1.output_height);
	printf("Image color components: %i\n", cinfo1.out_color_components);	// 3 = RGB, 1 = Grayscale
	printf("Image components: %i\n", cinfo1.output_components);
	//printf("Image height: %d\n", cinfo1.colormap);
	//printf("Image height: %d\n", cinfo1.actual_number_of_colors);	

	int width = cinfo1.output_width;
	int height = cinfo1.output_height;
	int components = cinfo1.out_color_components; // 3 = RGB, 1 = Grayscale



	//unsigned char* buffer = new unsigned char *[cinfo.output_width * cinfo.output_height * 3];

	// Figure out how to acces data
	int numberOfLines = 0;
	while (cinfo1.output_scanline < cinfo1.output_height) {
		numberOfLines = jpeg_read_scanlines(&cinfo1, buffer, 1);
		//put_scanline_someplace(buffer[0], row_stride1);
		//buffer[numberOfLines*row_stride1];
	}

	// unsigned char* line;
	// int numberOfSamples;
	// while (cinfo.output_scanline < cinfo.output_height) {
	// 	numberOfSamples = jpeg_read_scanline(&cinfo, (JSAMPARRAY) );
	// }

	jpeg_finish_decompress(&cinfo1);
	jpeg_destroy_decompress(&cinfo1);
	fclose(infile);

	// Move data from unsiged char ** to unsigned char *
	//printf("(%i, %i, %i)", buffer[0][0], buffer[0][1], buffer[0][2]);








	/*********** Compressing and writing **************/
	
	// Works, but colorspace is incorrect? Everything is blue
	//unsigned char image_buffer[] =  {255,0,0, 0,255,0, 0,0,255, 0,0,0};

	struct jpeg_compress_struct cinfo;
	struct jpeg_error_mgr jerr;
	cinfo.err = jpeg_std_error(&jerr);
	jpeg_create_compress(&cinfo);

	FILE * outfile;
	if ((outfile = fopen("output.jpeg", "wb")) == NULL) {
		fprintf(stderr, "Can't open %s\n","output.jpeg" );
		return 0;
	}
	jpeg_stdio_dest(&cinfo, outfile);

	// Require info
	// int width = 2;
	// int height = 2;
	// int components = 3; // 3 = RGB, 1 = Grayscale
	//JCS_RGB or JCS_GRAYSCALE
	cinfo.image_width = width;
	cinfo.image_height = height;
	cinfo.input_components = components;
	cinfo.in_color_space = JCS_RGB;
	jpeg_set_defaults(&cinfo);

	// Optional parameter settings 
	// int quality = 15 // 1-100 or 0-100?
	// jpeg_set_quality(&cinfo, quality, TRUE);

	jpeg_start_compress(&cinfo, TRUE);



	JSAMPROW row_pointer[1];
	int row_stride;
	row_stride = width * 3; // JSAMPLEs per row in image (1 if grayscale?)
	
	// Need to figure out how to pass data to writer!
	while(cinfo.next_scanline < cinfo.image_height) {
		row_pointer[0] = & buffer[0][cinfo.next_scanline * row_stride];
		jpeg_write_scanlines(&cinfo, row_pointer, 1);
	}

	jpeg_finish_compress(&cinfo);
	fclose(outfile);

	jpeg_destroy_compress(&cinfo);
	
	


	return 0;
}
